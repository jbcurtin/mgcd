use std::collections::VecDeque;

use crate::gcd;
use crate::util;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use std::sync::Arc;

pub async fn calculate() {
  // Load factors
  let filepath = util::obtain_filepath(false).await;
  let filepath = std::path::Path::new(&filepath);
  let factors = tokio::fs::read_to_string(filepath).await.unwrap();
  let factors = factors.split("\n")
    .into_iter().map(|fs| {
      fs.to_string().split(",").map(|f|f.to_string()).collect::<Vec<String>>()
    })
    .collect::<Vec<Vec<String>>>();
  // Push factors into shared Queue
  let factor_queue = Arc::new(Mutex::new(VecDeque::new()));
  for factor_set in factors {
    if factor_set.first().unwrap() == "" {
      continue;
    }
    let factor_set = factor_set.into_iter().map(|fac|fac.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    factor_queue.lock().await.push_back(factor_set);
  }
  let mut joiners = JoinSet::new();
  joiners.spawn(async move {
    loop {
      let mut queue = factor_queue.lock().await;
      if queue.len() < 1 {
        break
      } else {
        let factorable = queue.pop_front().unwrap();
        match gcd::mgcd(factorable.clone()) {
          Some(factor) => tracing::info!("GCD Found {factor:?} - {factorable:?}"),
          None => {}
        }
      }
      drop(queue);
    }
  });
  while let Some(result) = joiners.join_next().await {
    match result {
      Ok(_) => {},
      Err(_err) => {}
    }
  }
  // Wrap factor_queue in a Mutex
  
  // let inputs = vec![1071, 462];
  // let result = gcd::mgcd(inputs);
  // println!("Result {result:?}");
}