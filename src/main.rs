use error_chain::error_chain;
use reqwest::StatusCode;
use std::path::Path;
use structopt::clap::arg_enum;
use structopt::StructOpt;

mod file_loader;
mod hf_uri;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

arg_enum! {
    #[derive(Debug)]
    enum CliAction {
        Copy
    }
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct CliArgs {
    /// The Hugging Face model uri, e.g. hf://philschmid/infinity-sentiment
    hf_uri: String,

    /// The Hugging Face model uri, e.g. hf://philschmid/infinity-sentiment
    destination_dir: Option<String>,

    /// Revision of the repository, default is main
    #[structopt(short = "r", long = "revision", default_value = "main")]
    revision: String,

    /// Wether auth token should be used or not
    #[structopt(short = "t", long = "token")]
    use_auth_token: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::from_args();

    let repository: hf_uri::Repository = match hf_uri::parse(&args.hf_uri) {
        Ok(repository) => repository,
        Err(err) => panic!(err),
    };

    let response = reqwest::blocking::get(&repository.url)?;

    // Parsing the json manually since the response.json changes based on tags, cannot guarantee structure
    let mut repository_information: serde_json::Value = match response.status() {
        StatusCode::OK => response.json()?,
        StatusCode::NOT_FOUND => {
            panic!("File {} not found", &response.url());
        }
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized to load file {}.", &response.url());
        }
        err => panic!("Received response status: {:?}", err),
    };

    let siblings: Vec<file_loader::Sibling> =
        serde_json::from_value(repository_information["siblings"].take()).unwrap();

    let filtered_siblings = match &repository.filter {
        Some(filter) => file_loader::filter_siblings(siblings, &filter),
        None => siblings,
    };

    for file in filtered_siblings {
        let remote_file_url =
            hf_uri::UrlCreater::create_remote_file_url(&repository.name, &file.rfilename);
        // adjusts save path for filter directory to remove unnecessary structure
        let file_name_path = match &repository.filter {
            // matches if only 1 file should be loaded, then load it in the dest_dir
            Some(filter) => match Path::new(filter).extension() {
                Some(_) => Path::new(Path::new(filter).file_name().unwrap()),
                None => Path::new(file.rfilename.as_str())
                    .strip_prefix(filter)
                    .unwrap(),
            },
            None => Path::new(file.rfilename.as_str()),
        };

        match file_loader::download_and_save_file(
            remote_file_url,
            &file_name_path,
            args.destination_dir.clone(),
        ) {
            Ok(_) => (),
            Err(why) => panic!("couldn't load file {}, ", why),
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_ilter_siblings() {
//         let test_siblings: Vec<Sibling> = vec![
//             Sibling {
//                 rfilename: String::from("infinity/tokenizers/tokenizer.json"),
//             },
//             Sibling {
//                 rfilename: String::from("infinity/tokenizers/tokenizer.json"),
//             },
//             Sibling {
//                 rfilename: String::from("huggingface/tokenizer.json"),
//             },
//             Sibling {
//                 rfilename: String::from("tokenizers/tokenizer.json"),
//             },
//         ];
//         let test_include_filter_path = vec![String::from("infinity"), String::from("huggingface")];
//         let test_output = filter_siblings(test_siblings, test_include_filter_path);
//         assert_eq!(
//             test_output,
//             vec![
//                 Sibling {
//                     rfilename: String::from("infinity/tokenizers/tokenizer.json"),
//                 },
//                 Sibling {
//                     rfilename: String::from("infinity/tokenizers/tokenizer.json"),
//                 },
//                 Sibling {
//                     rfilename: String::from("huggingface/tokenizer.json"),
//                 }
//             ],
//         );
//     }
//     #[test]
//     #[should_panic]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }
