#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SummaryProcess {
   pub file_name_upload : String,
   pub text: String
}