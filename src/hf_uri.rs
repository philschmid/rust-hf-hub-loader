use url::{ParseError, Position, Url};
#[derive(Debug)]
pub struct Repository {
  pub name: String,
  pub url: String,
  pub filter: Option<String>,
}

pub fn parse(hf_uri: &str) -> Result<Repository, ParseError> {
  let parsed_hf_uri = Url::parse(hf_uri)?;

  assert_eq!(parsed_hf_uri.scheme(), "hf");
  match parsed_hf_uri.scheme() {
    "hf" => {}
    _ => panic!(
      "Schema {} is not supported you need to provide a hf uri",
      parsed_hf_uri.scheme()
    ),
  }

  let uri_without_schema: &str = &parsed_hf_uri[Position::BeforeHost..];

  let repository: Repository = match uri_without_schema.contains("//") {
    true => {
      let url_split: Vec<&str> = uri_without_schema.split("//").collect();
      Repository {
        name: String::from(url_split[0]),
        url: UrlCreater::create_remote_repository_url(&url_split[0]),
        filter: Some(String::from(url_split[1])),
      }
    }
    _ => Repository {
      name: String::from(uri_without_schema),
      url: UrlCreater::create_remote_repository_url(&uri_without_schema),
      filter: None,
    },
  };

  Ok(repository)
}

pub struct UrlCreater {}

impl UrlCreater {
  const BASE_HF_URL: &'static str = "https://huggingface.co";

  pub fn create_remote_repository_url(repository: &str) -> String {
    format!(
      "{base_url}/api/models/{repo}",
      base_url = UrlCreater::BASE_HF_URL,
      repo = repository,
    )
  }
  pub fn create_remote_file_url(repository: &str, file_path: &str) -> String {
    format!(
      "{base_url}/{repo}/raw/main/{file_path}",
      base_url = UrlCreater::BASE_HF_URL,
      repo = repository,
      file_path = file_path,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn test_uri_schema() {
    let hf_uri = "xs://philschmid/infinity-sentiment";
    match parse(hf_uri) {
      Ok(res) => println!("{:?}", res),
      Err(err) => panic!(err),
    }
  }

  #[test]
  fn test_uri_parser_without_filter() {
    let hf_uri = "hf://philschmid/infinity-sentiment";
    match parse(hf_uri) {
      Ok(repository) => {
        assert_eq!(
          repository.name,
          String::from("philschmid/infinity-sentiment")
        );
        assert_eq!(
          repository.url,
          String::from("https://huggingface.co/api/models/philschmid/infinity-sentiment")
        );
        match repository.filter {
          None => {}
          _ => panic!("should be None in the test"),
        }
      }
      Err(err) => panic!("{}", err),
    }
  }
  #[test]
  fn test_uri_parser_with_filter() {
    let hf_uri = "hf://philschmid/infinity-sentiment//infinity/config.json";
    match parse(hf_uri) {
      Ok(repository) => {
        assert_eq!(
          repository.name,
          String::from("philschmid/infinity-sentiment")
        );
        assert_eq!(
          repository.url,
          String::from("https://huggingface.co/api/models/philschmid/infinity-sentiment")
        );
        match repository.filter {
          None => {}
          Some(filter) => assert_eq!(filter, "infinity/config.json"),
        }
      }
      Err(err) => panic!("{}", err),
    }
  }
  #[test]
  fn test_create_remote_repository_url() {
    let test_url = "https://huggingface.co/api/models/philschmid/infinity-sentiment";
    let created_url = UrlCreater::create_remote_repository_url("philschmid/infinity-sentiment");
    assert_eq!(test_url, created_url)
  }
  #[test]
  fn test_create_remote_file_url() {
    let test_url =
      "https://huggingface.co/philschmid/infinity-sentiment/raw/main/infinity/config.json";
    let created_url =
      UrlCreater::create_remote_file_url("philschmid/infinity-sentiment", "infinity/config.json");
    assert_eq!(test_url, created_url)
  }
}
