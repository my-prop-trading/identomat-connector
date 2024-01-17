use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccessTokenResponse {
    #[serde(rename = "session_token")]
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeneralDocument {
    #[serde(rename = "documentType")]
    pub document_type: String,
    pub pages: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RejectReason {
    pub value: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    #[serde( skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub local_last_name: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub birthday_time: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub birth_place: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub document_issued: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub document_expires: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub document_expires_time: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde( skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Questionnaire {
    pub key: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub key: String,
    pub answer: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScreeningMatch {
    pub score: f64,
    pub name: String,
    #[serde(rename = "personId")]
    pub person_id: String,
    #[serde(rename = "reviewStatus")]
    pub review_status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TechnicalDetails {
    #[serde(rename = "remoteAddress")]
    pub remote_address: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

pub enum ResultType {
    Approved = 0,
    Rejected = 1, 
    ManualCheck = 2,
    DisagreedWithTerms = 4,
    InProgress = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicantData {
    pub result: String,
    pub live: bool,
    pub similarity: i64,
    pub document_type: String,
    #[serde(rename = "generalDocuments")]
    pub general_documents: Vec<GeneralDocument>,
    pub reject_reason: Option<RejectReason>,
    pub person: Person,
    #[serde(rename = "configId")]
    pub config_id: Option<String>,
    #[serde(rename = "trustedDatabaseRecord")]
    pub trusted_database_record: Option<serde_json::Value>,
    pub result_comment: String,
    pub questionnaires: Option<Vec<Questionnaire>>,
    #[serde(rename = "screeningMatches")]
    pub screening_matches: Option<Vec<ScreeningMatch>>,
    #[serde(rename = "technicalDetails")]
    pub technical_details: Option<TechnicalDetails>,
}