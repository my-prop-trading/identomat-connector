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
    pub similarity: f64,
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


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_json_body() {
        let json_body = r#"
        {
            "result":"rejected",
            "similarity":0.6550394743680953,
            "live":true,
            "document_type":"id",
            "generalDocuments":[
               
            ],
            "reject_reason":{
               "value":"face_mismatch",
               "description":""
            },
            "result_comment":"3123",
            "name":"SPECIMEN SPECIMEN",
            "face_images":1,
            "id_card_front":{
               "Given_Names_en_US":"MARIANA",
               "Surname_en_US":"TKACHENKO",
               "Given_Names_ka_GE":"\u0410\u0420\u042f\u041d\u0410",
               "Surname_ka_GE":"\u0422\u041a\u0410\u0427\u0415\u041d\u041a\u041e",
               "Nationality_Code_en_US":"UKR",
               "Sex_en_US":"F",
               "Document_Number_en_US":"000000000",
               "Nationality_en_US":"UKR",
               "Issuing_State_Code_en_US":"UKR",
               "localMiddleName":"\u0406\u0412\u0410\u041d\u0407\u0412\u041d\u0410"
            },
            "id_card_back":{
               "Personal_Number_en_US":"05781305459",
               "Issuing_State_Code_en_US":"HRV",
               "Authority_en_US":"PU SPLITSKO",
               "Address_en_US":"SPLIT SPLIT EZA MUTIMIRA 2",
               "Given_Names_en_US":"SPECIMEN",
               "Surname_en_US":"SPECIMEN",
               "Nationality_Code_en_US":"HRV",
               "Sex_en_US":"F",
               "Date_of_Birth_en_US":"11\/25\/1979",
               "Date_of_Expiry_en_US":"8\/2\/2026",
               "Document_Number_en_US":"115501830",
               "Nationality_en_US":"HRV",
               "Date_of_Birth_ISO":"1979-11-25T00:00:00.000Z",
               "Date_of_Expiry_ISO":"2026-08-02T00:00:00.000Z",
               "mrz":"IOHRV115501830605781305459<<<<\n7911255F2608020HRV<<<<<<<<<<<5\nSPECIMEN<<SPECIMEN<<<<<<<<<<<<"
            },
            "suggested":{
               
            },
            "person":{
               "local_first_name":"\u0410\u0420\u042f\u041d\u0410",
               "local_last_name":"\u0422\u041a\u0410\u0427\u0415\u041d\u041a\u041e",
               "first_name":"SPECIMEN",
               "last_name":"SPECIMEN",
               "nationality":"HRV",
               "document_number":"115501830",
               "issuing_state":"HRV",
               "sex":"F",
               "local_middle_name":"\u0406\u0412\u0410\u041d\u0407\u0412\u041d\u0410",
               "birthday":"11\/25\/1979",
               "birthday_time":"1979-11-25T00:00:00.000Z",
               "age":44,
               "document_expires":"8\/2\/2026",
               "document_expires_time":"2026-08-02T00:00:00.000Z",
               "personal_number":"05781305459",
               "authority":"PU SPLITSKO",
               "address":"SPLIT SPLIT EZA MUTIMIRA 2",
               "status":"FIELDS_MISMATCH",
               "mrz":"IOHRV115501830605781305459<<<<\n7911255F2608020HRV<<<<<<<<<<<5\nSPECIMEN<<SPECIMEN<<<<<<<<<<<<"
            },
            "technicalDetails":{
               "remoteAddress":"84.43.240.162",
               "userAgent":"Mozilla\/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/120.0.0.0 Safari\/537.36",
               "countryCode":"BG"
            }
         }
        "#;
        let person: ApplicantData = serde_json::from_str(json_body).unwrap();
        assert!(person.result =="rejected");
    }
}
