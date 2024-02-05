use std::collections::HashMap;
use crate::rest::config::IdentomatConfig;
use crate::rest::endpoints::IdentomatEndpoint;
use serde_json::json;
use super::errors::IdentomatError;
use super::models::ApplicantData;


#[derive(Clone)]
pub struct IdentomatRestClient {
    company_key: String,
    host: String,
    inner_client: reqwest::Client,
}

impl IdentomatRestClient {
    pub fn new(company_key: String) -> Self {
        Self::new_with_config(company_key, IdentomatConfig::default())
    }

    pub fn new_with_config(company_key: String, config: IdentomatConfig) -> Self {
        Self {
            company_key: company_key,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
        }
    }
    
    pub async fn poi_session_begin(&self) 
    -> Result<String, IdentomatError>{
        let json_data = json!({
            "company_key": &self.company_key,
            "flags": {
                //"return_url": "https://....com",
                "liveness": true,
                //"document_types": ["driver_license", "residence_license"],
                "skip_agreement": true,
                "allow_document_upload":true,
                "language": "en",
                "skip_desktop": false
            }
        });

        let url_with_query: String = format!(
            "{}{}",
            self.host,
            String::from(IdentomatEndpoint::SessionBegin),
        );
        let client = &self.inner_client;
        let response = client
            .get(&url_with_query)
            .json(&json_data)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?.trim_matches('"').to_string();
            println!("Successful response: {}", response_text);
            return Ok(response_text);
        } else {
            let error_message = response.text().await?;
            println!("received text: {:?}", error_message);
            return Err(IdentomatError{
                message: error_message
            });
        }
    }


    pub async fn poa_session_begin(&self) 
    -> Result<String, IdentomatError> {
        let json_data = json!({
            "company_key": &self.company_key,
            "flags": {
                //"return_url": "https://....com",
                "liveness": false,
                "document_types": [],
                "general_document_types": ["utility_bill", "bank_statement"],
                "allow_general_document_upload": true,
                "skip_agreement": true,
                "skip_document": true,
                "skip_face": true,
                "allow_general_document_capture": true,
                "language": "en",
            }
        });

        let url_with_query: String = format!(
            "{}{}",
            self.host,
            String::from(IdentomatEndpoint::SessionBegin),
        );
        let client = &self.inner_client;
        let response = client
            .post(&url_with_query)
            .json(&json_data)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?.trim_matches('"').to_string();
            println!("Successful response: {}", response_text);
            return Ok(response_text);
        } else {
            let error_message = response.text().await?;
            println!("received text: {:?}", error_message);
            return Err(IdentomatError{
                message: error_message
            });
        }
    }

    pub async fn get_applicant_data(
        &self,
        session_token: impl Into<String>,
    ) -> Result<ApplicantData, IdentomatError> {

        let url_with_query: String = format!(
            "{}{}",
            self.host,
            String::from(IdentomatEndpoint::SessionResult),
        );
        let client = &self.inner_client;

        let mut form_data = HashMap::new();
        form_data.insert("company_key".to_owned(), self.company_key.to_owned());
        form_data.insert("session_token".to_owned(), session_token.into());

        let response = client
            .post(&url_with_query)
            .form(&form_data)
            .send()
            .await
            ?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            println!("Successful response: {}", response_text);
            let response: ApplicantData = serde_json::from_str(response_text.as_str()).unwrap();
            return Ok(response);
        } else {
            let error_message = response.text().await?;
            println!("received text: {:?}", error_message);
            return Err(IdentomatError{
                message: error_message
            });
        }
    }

}
