extern crate bgpq3;

pub fn main() -> std::io::Result<()> {
    let networks = bgpq3::Bgpq3::new().query_v6("AS-RAPPET")?;
    println!("{:?}", networks);

    Ok(())
}