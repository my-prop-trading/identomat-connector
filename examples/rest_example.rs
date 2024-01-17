use identomat_connector::rest::config::IdentomatConfig;
use identomat_connector::rest::rest_client::IdentomatRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let company_key = std::env::var("COMPANY_KEY").unwrap();
    let client = IdentomatRestClient::new_with_config(company_key, IdentomatConfig::test_env());
    let client_id = Uuid::new_v4().to_string();
    let poi_token = get_poi_token(&client, client_id.as_str()).await;
    let _ = get_applicant(&client, poi_token.as_str()).await;

    let poa_token = get_poa_token(&client, client_id.as_str()).await;
    let _ = get_applicant(&client, poa_token.as_str()).await;
}

async fn get_poi_token(client: &IdentomatRestClient, client_id: &str) -> String {
    let access_token = client.poi_session_begin().await.unwrap_or_default();

    println!("get_poi_token {} result: {}", client_id, access_token);
    access_token
}

async fn get_poa_token(client: &IdentomatRestClient, client_id: &str) -> String {
    let access_token = client.poa_session_begin().await.unwrap_or_default();

    println!("get_poa_token {} result: {}", client_id, access_token);
    access_token
}

async fn get_applicant(client: &IdentomatRestClient, session_token: &str) {
    let applicant = client.get_applicant_data(session_token).await;

    println!("get_applicant result: {:?}", applicant);
}
