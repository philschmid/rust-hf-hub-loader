use error_chain::error_chain;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
  }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Sibling {
  pub rfilename: String,
}

pub fn download_and_save_file(
  file_url: String,
  destination_file_path: &Path,
  auth_token: &str,
  destination_directory: Option<String>,
) -> Result<()> {
  let path = match destination_directory {
    Some(dest_dir) => Path::new(&dest_dir).join(&destination_file_path),
    None => Path::new(&destination_file_path).to_path_buf(),
  };

  println!("Loading: {}", file_url);

  let client = reqwest::blocking::Client::new();
  let response = client.get(file_url)
  .header("authorization", format!("Bearer {token}", token=auth_token))
  .send()?;

  let content = match response.status() {
    StatusCode::OK => response.text()?,
    StatusCode::NOT_FOUND => {
      panic!("File {} not found", &response.url());
    }
    StatusCode::UNAUTHORIZED => {
      panic!("Unauthorized to load file {}.", &response.url());
    }
    err => panic!("Received response status: {:?}", err),
  };

  match path.parent() {
    Some(parent) if parent != Path::new("") => {
      if !parent.exists() {
        fs::create_dir(parent)?;
      }
    }
    _ => {}
  };

  let mut file = File::create(&path)?;
  file.write_all(content.as_bytes())?;
  Ok(())
}

pub fn filter_siblings(siblings: Vec<Sibling>, filter_string: &str) -> Vec<Sibling> {
  siblings
    .into_iter()
    .filter(|sibling| sibling.rfilename.as_str().contains(filter_string))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use temp_testdir::TempDir;
  #[test]
  fn test_download_and_save_file() {
    let temp = TempDir::default();
    let file_path = temp.join("infinity/config.json");
    let file_url = String::from(
      "https://huggingface.co/philschmid/infinity-sentiment/raw/main/infinity/config.json",
    );
    match download_and_save_file(file_url, Path::new(file_path.to_str().unwrap()),"none", None) {
      Ok(_) => {}
      Err(err) => panic!("{}",err),
    }
    println!("{:?}", temp.join("infinity/config.json"));
    let contents = fs::read_to_string(temp.join("infinity/config.json")).unwrap();
    assert_eq!(file_path.exists(), true);
    assert_ne!(contents.len(), 0);
  }

  #[test]
  #[should_panic]
  fn test_download_of_file_which_doesnt_exist() {
    let temp = TempDir::default();
    let file_path = temp.join("infinity/config2.json");
    let file_url = String::from(
      "https://huggingface.co/philschmid/infinity-sentiment/raw/main/infinity/config2.json",
    );
    match download_and_save_file(file_url, Path::new(file_path.to_str().unwrap()),"none", None) {
      Ok(_) => {}
      Err(err) => panic!("{}",err),
    }
  }

  #[test]
  fn test_download_of_private() {
    let temp = TempDir::default();
    let file_path = temp.join("config.json");
    let file_url = String::from(
      "  https://huggingface.co/philschmid/private-repo-test/resolve/main/config.json",
    );
    match download_and_save_file(file_url, Path::new(file_path.to_str().unwrap()),"xxx", None) {
      Ok(_) => {}
      Err(err) => panic!("{}",err),
    }
  }

  #[test]
  fn test_filter_siblings() {
    let test_siblings: Vec<Sibling> = vec![
      Sibling {
        rfilename: String::from("infinity/tokenizers/tokenizer.json"),
      },
      Sibling {
        rfilename: String::from("infinity/tokenizers/tokenizer.json"),
      },
      Sibling {
        rfilename: String::from("huggingface/tokenizer.json"),
      },
      Sibling {
        rfilename: String::from("tokenizers/tokenizer.json"),
      },
    ];

    let filter_string = "infinity";

    let test_output = filter_siblings(test_siblings, filter_string);
    assert_eq!(
      test_output,
      vec![
        Sibling {
          rfilename: String::from("infinity/tokenizers/tokenizer.json"),
        },
        Sibling {
          rfilename: String::from("infinity/tokenizers/tokenizer.json"),
        }
      ],
    );
  }
}

