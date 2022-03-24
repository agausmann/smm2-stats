use reqwest::{Client, Error};

pub const OFFICIAL_BASE_URL: &str = "https://tgrcode.com/mm2";

pub struct Api {
    client: Client,
    base_url: String,
}

impl Api {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_level_data(&self, course_id: &str) -> Result<Vec<u8>, Error> {
        let response = self
            .client
            .get(format!("{}/level_data/{}", self.base_url, course_id))
            .send()
            .await?
            .error_for_status()?;
        let bytes = response.bytes().await?;
        Ok(bytes.as_ref().to_vec())
    }
}

impl Default for Api {
    fn default() -> Self {
        Self::new(OFFICIAL_BASE_URL.into())
    }
}
