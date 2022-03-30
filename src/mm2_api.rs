use std::time::Duration;

use reqwest::{Client, ClientBuilder, Error};
use serde::{Deserialize, Serialize};

pub const OFFICIAL_BASE_URL: &str = "https://tgrcode.com/mm2";

pub struct Api {
    client: Client,
    base_url: String,
}

impl Api {
    pub fn new(base_url: String) -> Result<Self, Error> {
        Ok(Self {
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(120))
                .build()?,
            base_url,
        })
    }

    pub fn official_server() -> Result<Self, Error> {
        Self::new(OFFICIAL_BASE_URL.into())
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

    pub async fn search_endless_mode(
        &self,
        count: u16,
        difficulty: Difficulty,
    ) -> Result<Vec<Course>, Error> {
        let response = self
            .client
            .get(format!(
                "{}/search_endless_mode?count={}&difficulty={}",
                self.base_url,
                count,
                difficulty.api_str()
            ))
            .send()
            .await?
            .error_for_status()?;
        let parsed: SearchEndlessMode = response.json().await?;
        Ok(parsed.courses)
    }
}

pub enum Difficulty {
    Easy,
    Normal,
    Expert,
    SuperExpert,
}

impl Difficulty {
    fn api_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "e",
            Difficulty::Normal => "n",
            Difficulty::Expert => "ex",
            Difficulty::SuperExpert => "sex",
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SearchEndlessMode {
    courses: Vec<Course>,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Course {
    pub name: String,
    pub description: String,
    pub course_id: String,
}
