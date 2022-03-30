use smm2_stats::mm2_api::Api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let course_id = args.next().expect("missing argument [course id]");
    let outfile = args.next().unwrap_or_else(|| course_id.clone());

    let api = Api::official_server()?;
    let level_data = api.get_level_data(&course_id).await?;
    tokio::fs::write(outfile, level_data).await?;
    Ok(())
}
