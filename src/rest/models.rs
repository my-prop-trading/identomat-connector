use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccessTokenResponse {
    #[serde(rename = "session_token")]
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Final {
    #[serde(rename = "documentIssued", skip_serializing_if = "Option::is_none")]
    pub document_issued: Option<String>,
    #[serde(rename = "formattedAddress", skip_serializing_if = "Option::is_none")]
    pub formatted_address: Option<String>,
    #[serde(rename = "issuingAuthority", skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<String>,
    #[serde(rename = "documentExpires", skip_serializing_if = "Option::is_none")]
    pub document_expires: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DocumentPage {
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    #[serde(rename = "typeId")]
    pub type_id: String,
    #[serde(rename = "final")]
    pub page_final: Final,
    #[serde(rename = "documentPages")]
    pub document_pages: Vec<DocumentPage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeneralDocument {
    #[serde(rename = "documentType")]
    pub document_type: String,
    #[serde(rename = "pages")]
    pub pages: Vec<Page>,
    #[serde(rename = "typeId")]
    pub type_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RejectReason {
    pub value: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_issued: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_expires_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    fn test_poi_json_body() {
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
               "Given_Names_en_US":"TEST",
               "Surname_en_US":"USER",
               "Given_Names_ka_GE":"\u0054\u0045\u0053\u0054",
               "Surname_ka_GE":"\u0055\u0053\u0045\u0052",
               "Nationality_Code_en_US":"GER",
               "Sex_en_US":"F",
               "Document_Number_en_US":"000000000",
               "Nationality_en_US":"GER",
               "Issuing_State_Code_en_US":"GER",
               "localMiddleName":"EMPTY"
            },
            "id_card_back":{
               "Personal_Number_en_US":"11223344555",
               "Issuing_State_Code_en_US":"HRV",
               "Authority_en_US":"AUTHORITY",
               "Address_en_US":"STREET ADDRESS 1",
               "Given_Names_en_US":"SPECIMEN",
               "Surname_en_US":"SPECIMEN",
               "Nationality_Code_en_US":"HRV",
               "Sex_en_US":"F",
               "Date_of_Birth_en_US":"11\/25\/1979",
               "Date_of_Expiry_en_US":"8\/2\/2026",
               "Document_Number_en_US":"001122333",
               "Nationality_en_US":"HRV",
               "Date_of_Birth_ISO":"1979-11-25T00:00:00.000Z",
               "Date_of_Expiry_ISO":"2026-08-02T00:00:00.000Z",
               "mrz":"IOHRV0011223336057813XXXXX<<<<\n1111222F2XXXXXHRV<<<<<<<<<<<5\nSPECIMEN<<SPECIMEN<<<<<<<<<<<<"
            },
            "suggested":{
               
            },
            "person":{
               "local_first_name":"\u0410\u0420\u042f\u041d\u0410",
               "local_last_name":"\u0422\u041a\u0410\u0427\u0415\u041d\u041a\u041e",
               "first_name":"SPECIMEN",
               "last_name":"SPECIMEN",
               "nationality":"HRV",
               "document_number":"001122333",
               "issuing_state":"HRV",
               "sex":"F",
               "local_middle_name":"\u0406\u0412\u0410\u041d\u0407\u0412\u041d\u0410",
               "birthday":"11\/25\/1979",
               "birthday_time":"1979-11-25T00:00:00.000Z",
               "age":44,
               "document_expires":"8\/2\/2026",
               "document_expires_time":"2026-08-02T00:00:00.000Z",
               "personal_number":"11223344555",
               "authority":"AUTHORITY",
               "address":"STREET ADDRESS 1",
               "status":"FIELDS_MISMATCH",
               "mrz":"IOHRV001122333611223344555<<<<\n1111222F2XXXXXHRV<<<<<<<<<<<5\nSPECIMEN<<SPECIMEN<<<<<<<<<<<<"
            },
            "technicalDetails":{
               "remoteAddress":"127.0.0.1",
               "userAgent":"Mozilla\/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/120.0.0.0 Safari\/537.36",
               "countryCode":"BG"
            }
         }
        "#;
        let person: ApplicantData = serde_json::from_str(json_body).unwrap();
        assert!(person.result == "rejected");
    }

    #[test]
    fn test_poa_json_body() {
        let json_body = r#"
        {            
            "result":"approved",
            "similarity":0,
            "live":false,
            "document_type":"",
            "generalDocuments":[
                {
                    "pages":[
                        {
                        "typeId":"utility-bill",
                        "final":{
                            "documentIssued":"2024-01-25T22:00:00.000Z",
                            "formattedAddress":"Fill address 39",
                            "issuingAuthority":"Fill organisation A",
                            "documentExpires":"",
                            "fullName":"test user"
                        },
                        "documentPages":[
                            {
                                "pageNumber":1
                            }
                        ]
                        }
                    ],
                    "documentType":"UTILITY_BILL",
                    "typeId":"utility_bill"
                }
            ],
            "reject_reason":{
                "value":"",
                "description":""
            },
            "result_comment":"done",
            "name":"",
            "face_images":0,
            "suggested":{
                
            },
            "person":{
                
            },
            "technicalDetails":{
                "remoteAddress":"127.0.0.1",
                "userAgent":"Mozilla\/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit\/537.36 (KHTML, like Gecko) Chrome\/120.0.0.0 Safari\/537.36",
                "countryCode":"BG"
            }
            }
        "#;
        let person: ApplicantData = serde_json::from_str(json_body).unwrap();
        assert!(person.result == "approved");
    }

    #[test]
    fn test_poa_json_body_document_issued_optional() {
        let json_body = r#"
            {
                "result": "approved",
                "similarity": 0,
                "live": false,
                "document_type": "",
                "generalDocuments": [
                    {
                        "pages": [
                            {
                                "typeId": "bank-statement",
                                "final": {
                                    "street": "Test Tokoev Street",
                                    "state": "Bishkek City",
                                    "city": "Bishkek",
                                    "country": "Kyrgyzstan",
                                    "postalCode": "720000",
                                    "streetNumber": "99",
                                    "countryCode": "KGZ",
                                    "authority": "\u043e\u0430\u043e \u00ab\u043e\u043f\u0442\u0438\u043c\u0430 \u0431\u0430\u043d\u043a\u00bb \u0430\u0430\u043a \u00aboptima banke ojsc",
                                    "address": "test tokoeva street, 99\/2",
                                    "formattedAddress": "99 Test Tokoev St, Bishkek 720000, Kyrgyzstan",
                                    "latitude": 42.8248407,
                                    "longitude": 74.61463230000001,
                                    "fullName": "yurets ogurets",
                                    "firstName": "yurets",
                                    "lastName": "ogurets",
                                    "registrationNumber": "7698",
                                    "permanent": null,
                                    "county": null,
                                    "sublocality": "Oktyabr District",
                                    "houseNumber": null,
                                    "postalTown": null,
                                    "localType": "ROOFTOP",
                                    "documentIssuedTime": "2024-07-23T00:00:00.000Z"
                                },
                                "documentPages": [
                                    {
                                        "pageNumber": 1
                                    }
                                ],
                                "pageNumber": 1,
                                "statuses": [
                                    "issued-date-was-not-found"
                                ]
                            }
                        ],
                        "documentType": "BANK_STATEMENT",
                        "permanent": null,
                        "typeId": "bank_statement"
                    }
                ],
                "reject_reason": {
                    "value": "",
                    "description": ""
                },
                "result_comment": "approve in database",
                "name": "yurets ogurets",
                "face_images": 0,
                "suggested": {},
                "person": {
                    "address": "53 Test Tokoev St, Bishkek 720000, Kyrgyzstan"
                },
                "technicalDetails": {
                    "remoteAddress": "127.0.0.1",
                    "userAgent": "Mozilla\/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit\/605.1.15 (KHTML, like Gecko) Version\/17.5 Safari\/605.1.15",
                    "countryCode": "KG"
                }
            }
        "#;
        let person: ApplicantData = serde_json::from_str(json_body).unwrap();
        assert!(person.result == "approved");
    }
}
