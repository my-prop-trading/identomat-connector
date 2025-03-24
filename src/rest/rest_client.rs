use std::collections::HashMap;
use crate::rest::config::IdentomatConfig;
use crate::rest::endpoints::IdentomatEndpoint;
use flurl::{FlUrl, FlUrlError};
use http::StatusCode;
use serde_json::json;
use super::errors::IdentomatError;
use super::models::ApplicantData;


#[derive(Clone)]
pub struct IdentomatRestClient {
    company_key: String,
    host: String,
}

impl IdentomatRestClient {
    pub fn new(company_key: String) -> Self {
        Self::new_with_config(company_key, IdentomatConfig::default())
    }

    pub fn new_with_config(company_key: String, config: IdentomatConfig) -> Self {
        Self {
            company_key: company_key,
            host: config.rest_api_host,
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
        let mut debug_info = "".to_string();

        let client = FlUrl::new(url_with_query);
        let response = client
            .post_json_with_debug(&json_data, &mut debug_info)
            .await?;
        println!("Http request:{:?}", debug_info);

        let code = StatusCode::from_u16(response.get_status_code()).map_err(|e| {
            println!("Failed to read status result: {:?}", e);
            IdentomatError { message: e.to_string() }
        })?;

        let body = response.receive_body()
            .await
            .map_err(|e: FlUrlError| {
                println!("Failed to receive body: {:?}", e);
                IdentomatError { message: e.to_string() }
            })?;

        let parsed = String::from_utf8(body)
            .map_err(|e| {
                println!("Failed to convert from_utf8 body: {}", e);
                IdentomatError { message: e.to_string() }
            })?;

        if code.is_success() {
            println!("Successful response: {}", parsed);
            Ok(parsed)
        } else {
            println!("received text: {:?}", parsed);
            Err(IdentomatError{
                message: parsed
            })
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

        let mut debug_info = "".to_string();

        let client = FlUrl::new(url_with_query);
        let response = client
            .post_json_with_debug(&json_data, &mut debug_info)
            .await?;
        println!("Http request:{:?}", debug_info);

        let code = StatusCode::from_u16(response.get_status_code()).map_err(|e| {
            println!("Failed to read status result: {:?}", e);
            IdentomatError { message: e.to_string() }
        })?;

        let body = response.receive_body()
            .await
            .map_err(|e: FlUrlError| {
                println!("Failed to receive body: {:?}", e);
                IdentomatError { message: e.to_string() }
            })?;

        let parsed = String::from_utf8(body)
        .map_err(|e| {
            println!("Failed to convert from_utf8 body: {}", e);
            IdentomatError { message: e.to_string() }
        })?;

        if code.is_success() {
            println!("Successful response: {}", parsed);
            Ok(parsed)
        } else {
            println!("received text: {:?}", parsed);
            Err(IdentomatError{
                message: parsed
            })
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

        let mut form_data = HashMap::new();
        form_data.insert("company_key".to_owned(), self.company_key.to_owned());
        form_data.insert("session_token".to_owned(), session_token.into());

        let mut debug_info = "".to_string();

        let client = FlUrl::new(url_with_query);
        let body = serde_urlencoded::to_string(&form_data)
            .map_err(|e| {
                println!("Failed to encode to url: {:?}", e);
                IdentomatError { message: e.to_string() }
            })?;

        let response = client
            .post_with_debug(Some(body.into_bytes()), &mut debug_info)
            .await?;
        println!("Http request:{:?}", debug_info);

        let code = StatusCode::from_u16(response.get_status_code()).map_err(|e| {
            println!("Failed to read status result: {:?}", e);
            IdentomatError { message: e.to_string() }
        })?;

        let body = response.receive_body()
            .await
            .map_err(|e: FlUrlError| {
                println!("Failed to receive body: {:?}", e);
                IdentomatError { message: e.to_string() }
            })?;

        let parsed = String::from_utf8(body)
            .map_err(|e| {
                println!("Failed to convert from_utf8 body: {}", e);
                IdentomatError { message: e.to_string() }
            })?;

        if code.is_success() {
            println!("Successful response: {}", parsed);
            let response: ApplicantData = serde_json::from_str(parsed.as_str()).unwrap();
            Ok(response)
        } else {
            println!("received text: {:?}", parsed);
            Err(IdentomatError{
                message: parsed
            })
        }
    }

}
