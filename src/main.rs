mod calc;
mod gcd;
mod gen;
mod opt;
mod util;

use clap::Parser;

#[tokio::main]
async fn main() {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).unwrap();
  match opt::GCDCLI::parse() {
    opt::GCDCLI::Calculate => calc::calculate().await,
    opt::GCDCLI::Generate(option) => gen::generate(option).await,
  }
}
