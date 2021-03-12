extern crate bgpq3;
extern crate tokio;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let networks = bgpq3::Bgpq3::new().tokio_query_v6("AS-RAPPET").await?;
    println!("{:?}", networks);

    Ok(())
}