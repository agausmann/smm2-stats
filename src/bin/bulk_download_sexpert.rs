use std::path::Path;

use smm2_stats::mm2_api::{Api, Difficulty};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::default();
    loop {
        let courses_result = api.search_endless_mode(300, Difficulty::SuperExpert).await;
        let courses = match courses_result {
            Ok(x) => x,
            Err(error) => {
                eprintln!("search error: {}", error);
                continue;
            }
        };

        for course in courses {
            println!("{} {}", course.course_id, course.name);
            let data = loop {
                let data_result = api.get_level_data(&course.course_id).await;
                match data_result {
                    Ok(data) => break data,
                    Err(error) => {
                        eprintln!("retry: {}", error);
                    }
                }
            };
            tokio::fs::write(Path::new("levels/sexpert/").join(&course.course_id), &data).await?;
        }
    }
}
