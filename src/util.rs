pub async fn obtain_filepath(rebuild: bool) -> String {
  let filepath = std::path::PathBuf::new().join(std::env::current_dir().unwrap());
  let filepath = filepath.join("data");
  if rebuild && tokio::fs::metadata(&filepath).await.is_ok() {
    tokio::fs::remove_dir_all(&filepath).await.unwrap();
  }
  if tokio::fs::metadata(&filepath).await.is_err() {
    tokio::fs::create_dir(&filepath).await.unwrap();
  }
  let filepath = filepath.join("numbers.txt");
  let filepath = filepath.as_path();
  filepath.to_string_lossy().to_string()
}