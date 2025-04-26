use clap;

#[derive(Debug, clap::Args, Clone)]
pub struct GenOption {
  #[arg(short, long, default_value_t=1000)]
  pub max_numbers: i64
}

#[derive(clap::Parser, Clone)]
#[command(name="gcd")]
#[command(bin_name="gcd")]
pub enum GCDCLI {
  Calculate,
  Generate(GenOption),
}