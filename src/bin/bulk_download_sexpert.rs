use std::path::Path;

use smm2_stats::mm2_api::{Api, Difficulty};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::default();
    let courses = api
        .search_endless_mode(200, Difficulty::SuperExpert)
        .await?;
    for course in courses {
        println!("{} {}", course.course_id, course.name);
        let data = api.get_level_data(&course.course_id).await?;
        tokio::fs::write(Path::new("sexpert/").join(&course.course_id), &data).await?;
    }
    Ok(())
}
