use std::{fmt::Display, future::Future, path::Path, time::Duration};

use smm2_stats::mm2_api::{Api, Difficulty};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::default();
    loop {
        let courses = retry_backoff(|| api.search_endless_mode(300, Difficulty::SuperExpert)).await;
        for course in courses {
            println!("{} {}", course.course_id, course.name);
            let data = retry_backoff(|| api.get_level_data(&course.course_id)).await;
            tokio::fs::write(Path::new("levels/sexpert/").join(&course.course_id), &data).await?;
        }
    }
}

async fn retry_backoff<T, E, F, G>(mut generator: F) -> T
where
    F: FnMut() -> G,
    G: Future<Output = Result<T, E>>,
    E: Display,
{
    let mut delay = Duration::from_secs(1);
    loop {
        match generator().await {
            Ok(x) => return x,
            Err(error) => {
                eprintln!("retry: {}", error);
                tokio::time::sleep(delay).await;
                delay = (delay * 2).max(Duration::from_secs(60));
            }
        }
    }
}
