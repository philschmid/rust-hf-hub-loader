use error_chain::error_chain;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use structopt::StructOpt;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Sibling {
    rfilename: String,
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct CliArgs {
    /// The Hugging Face model repository
    #[structopt(short = "r", long = "repository")]
    repository: String,

    /// list of strings, which will be used as filter to download the files
    #[structopt(
        short = "f",
        long = "filter",
        // default_value = "infinity",
        use_delimiter = true
    )]
    filter: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::from_args();
    println!("{:?}", args);
    let base_url = "https://huggingface.co";

    let request_url = format!(
        "https://huggingface.co/api/models/{repo}",
        repo = args.repository
    );
    let response = reqwest::get(&request_url).await?;
    // Parsing the json manually since the response.json changes based on tags, cannot guarantee structure
    let mut repository_information: serde_json::Value = response.json().await?;
    let siblings: Vec<Sibling> =
        serde_json::from_value(repository_information["siblings"].take()).unwrap();
    let filtered_siblings = filter_siblings(siblings, args.filter);

    for file in filtered_siblings {
        let remote_file_url = format!(
            "{base_url}/{repo}/resolve/main/{file_path}",
            base_url = base_url,
            repo = args.repository,
            file_path = file.rfilename,
        );
        load_file(remote_file_url, file.rfilename).await?
    }
    Ok(())
}

async fn load_file(file_url: String, file_path: String) -> Result<()> {
    let path = Path::new(&file_path);
    let directory = path.parent().unwrap();

    let requested_file = reqwest::get(file_url).await?;

    if !directory.exists() {
        fs::create_dir(directory)?;
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}, {:?}", why, path),
        Ok(file) => file,
    };
    let content = requested_file.text().await?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn filter_siblings(siblings: Vec<Sibling>, include_filter_path: Vec<String>) -> Vec<Sibling> {
    siblings
        .into_iter()
        .filter(|sibling| {
            include_filter_path
                .iter()
                .any(|path| sibling.rfilename.as_str().contains(path))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ftest_ilter_siblings() {
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
        let test_include_filter_path = vec![String::from("infinity"), String::from("huggingface")];
        let test_output = filter_siblings(test_siblings, test_include_filter_path);
        assert_eq!(
            test_output,
            vec![
                Sibling {
                    rfilename: String::from("infinity/tokenizers/tokenizer.json"),
                },
                Sibling {
                    rfilename: String::from("infinity/tokenizers/tokenizer.json"),
                },
                Sibling {
                    rfilename: String::from("huggingface/tokenizer.json"),
                }
            ],
        );
    }
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
}
