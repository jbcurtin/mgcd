use crate::opt::GenOption;
use crate::util;
use rand::prelude::*;
use tokio::io::AsyncWriteExt;


pub async fn generate(option: GenOption) {
  let max_nums = option.max_numbers;
  let mut samps = Vec::new();
  let mut rng = rand::rng();
  tracing::info!("Generating {max_nums:?} Numbers from a Normal Distribution");
  for _ in 0..max_nums {
    let samp = rng.sample(rand::distr::Uniform::new(1, 999_999_999).unwrap());
    samps.push(samp);
  }
  tracing::info!("Building number combinations with a min length of 2 and a max length of 9 ");
  let mut samp_ranges: Vec<Vec<i32>> = Vec::new();
  loop {
    let amount = rng.sample(rand::distr::Uniform::new(2, 9).unwrap());
    if amount + 1 > samps.len() {
      break
    }
    let samp_range = &samps.clone()[samps.len() - amount - 1..samps.len() - 1];
    let trunc_amount = samps.len() - amount - 1;
    samps.truncate(trunc_amount);
    samp_ranges.push(samp_range.to_vec())
  }
  // Write numbers to filepath
  let filepath = util::obtain_filepath(true).await;
  let filepath = std::path::Path::new(&filepath);
  let file = tokio::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(filepath)
    .await.unwrap();
  let mut buf_writer = tokio::io::BufWriter::new(file);
  for samp_range in samp_ranges.iter() {
    let message = samp_range.iter().map(|n| n.to_string()).collect::<Vec<String>>();
    let message = message.join(",");
    let message = format!("{message}\n");
    buf_writer.write(message.as_bytes()).await.unwrap();
  }
  tracing::info!("Numbers written to data/numbers.txt");
}