use std::{fmt::Display, future::Future, path::Path, time::Duration};

use smm2_stats::{
    course_decryptor::decrypt_course_data,
    mm2_api::{Api, Difficulty},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::official_server()?;
    let base_dir = Path::new("levels/sexpert/");
    loop {
        let courses = retry_backoff(|| api.search_endless_mode(300, Difficulty::SuperExpert)).await;
        for course in courses {
            let output_path = base_dir.join(&course.course_id);
            if output_path.exists() {
                continue;
            }

            println!("{} {}", course.course_id, course.name);
            let data = retry_backoff(|| api.get_level_data(&course.course_id)).await;
            let decrypted_data = decrypt_course_data(&data);
            let write_result = tokio::fs::write(output_path, &decrypted_data).await;
            match write_result {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("write failed: {}", error);
                }
            }
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
