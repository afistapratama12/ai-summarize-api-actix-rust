use actix_web::web;
use actix_multipart::Field;
use futures::StreamExt;
use pdf_extract;
use docx_rust::*;
use std::{
  error::Error, fs::File, io::{Read, Write}
};
use crate::{
  handler::request::FileSummary, 
  model::summary::SummaryProcess,
  libs::openai::chat_completion
};

pub struct FileService { }

impl FileService {
  pub fn new() -> Self {
    Self {}
  }

  pub async fn upload_file(&self, filepath: String, mut field: Field) -> Result<(), String> {
    let mut file = web::block(move || File::create(filepath.clone())).await.unwrap().unwrap();
    while let Some(chunk) = field.next().await {
      match chunk {
        Ok(data) => file.write_all(&data).unwrap(),
        Err(e) => return Err(format!("error reading file : {}", e)),
      }
    }

    Ok(())
  }

  pub async fn summarize(&self, files: Vec<FileSummary>, lang: String, input_text: Option<String>) -> Result<String, Box<dyn Error>> {
    let mut file_process = Vec::new();

    for file in files {
      match self.extract_text(&file.file_id, &file.file_ext).await {
        Ok(text) => {
          file_process.push(SummaryProcess{
            file_name_upload: file.file_name_upload.clone(),
            text: text,
          });
        }
        Err(err) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))),
      };
    }

    let chat_request = match lang.as_str() {
      "id" => self.build_chat_request_id(file_process, input_text.clone()),
      "en" => self.build_chat_request_en(file_process, input_text.clone()),
      _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "unsupported language, only allow language \"id\" and \"en\""))),
    };

    chat_completion(&chat_request).await
  }

  pub async fn extract_text(&self, file_id: &str, file_ext: &str) -> Result<String, String> {
    let file_name = format!("./upload/{}.{}", file_id, file_ext);

    let extracted_text = match file_ext {
      "pdf" => Ok(Self::extract_text_from_pdf(&file_name.clone())),
      "docx" => Ok(Self::extract_text_from_docx(&file_name.clone())),
      "txt" => Ok(Self::extract_text_from_txt(&file_name.clone())),
      _ => Err("Unsupported file type".to_string()),
    };

    match extracted_text {
      Ok(text) => text,
      Err(err) => Err(err),
    }
  }

  fn extract_text_from_pdf(file_path: &str) -> Result<String, String> {
    match pdf_extract::extract_text(file_path) {
      Ok(text) => Ok(text),
      Err(err) => Err(format!("Error reading pdf file: {}", err)),
    }
  }

  fn extract_text_from_docx(file_path: &str) -> Result<String, String> {
    let docx = DocxFile::from_file(file_path)
      .map_err(|e| format!("Error reading docx file: {}", e))?;

    let docx = docx.parse().map_err(|e| format!("error parse file docx: {}", e).to_string())?;

    let extracted_text = docx.document.body.text();

    Ok(extracted_text)
  }

  fn extract_text_from_txt(file_path: &str) -> Result<String, String> {
    let mut file = File::open(&file_path).map_err(|e| format!("Error reading txt file: {}", e))?;
    
    let mut contents = String::new();

    file.read_to_string(&mut contents).map_err(|e| format!("Error reading txt file: {}", e))?;
    
    Ok(contents)
  }

  pub fn build_chat_request_id(&self, file_process: Vec<SummaryProcess>, input_text: Option<String>) -> String {
    let mut text = "Saya telah mengekstrak teks dari file berikut:\n\n".to_string();
  
    for file in file_process {
      text.push_str(&format!("Nama file: {}\n", file.file_name_upload));
      text.push_str(&format!("Isi: {}\n\n", file.text));
    }
  
    text.push_str("tolong interaksi atau jawab dengan masukan berikut:\n\n");
  
    if let Some(input_text) = input_text {
      text.push_str(&format!("dapatkah kamu merangkum isi dari file tersebut dan {}", input_text));
    } else {
      text.push_str("dapatkah kamu merangkum isi dari file tersebut");
    }
  
    text
  }
  
  pub fn build_chat_request_en(&self, file_process: Vec<SummaryProcess>, input_text: Option<String>) -> String {
    let mut text  = if file_process.len() == 1 {
      "I have extracted the text from the following file:\n\n".to_string()
    } else {
      "I have extracted the text from the following files:\n\n".to_string()
    };
  
    for file in file_process {
      text.push_str(&format!("File name: {}\n", file.file_name_upload));
      text.push_str(&format!("Content: {}\n\n", file.text));
    }
  
    text.push_str("please interact or answer with this following input:\n\n");
  
    if let Some(input_text) = input_text {
      text.push_str(&format!("can you summarize the content of the file and {}", input_text));
    } else {
      text.push_str("can you summarize the content of the file");
    }
  
    text
  }

}