use clap::{App, Arg};
use std::io::{self, Write};
use std::fs::File;
use glob::glob;
use lnk_parser::LNKParser;
use winparsingtools::traits::Normalize;
use std::collections::HashMap;
use serde::Serialize;

enum OutputFormat {
    JSON,
    JSONL,
    CSV,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> OutputFormat {
        match s {
            "json" => OutputFormat::JSON,
            "jsonl" => OutputFormat::JSONL,
            "csv" => OutputFormat::CSV,
            _ => OutputFormat::CSV
        }
    }
}

fn parse_cli_args() -> clap::ArgMatches<'static>
{
    App::new("lnk_parser")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AbdulRhman Alfaifi - @A__ALFAIFI")
        .about("Windows LNK Files Parser")
        .arg(Arg::with_name("PATH")
                .short("-p")
                .long("--path")
                .takes_value(true)
                .multiple(true)
                .value_name("PATH")
                .help("Path(s) to LNK Metadata Files to be Parsed - accepts glob (Defaults to 'RecetItems' for all users)"))
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .default_value("stdout")
                .takes_value(true)
                .help("The file path to write the output to"))
        .arg(
            Arg::with_name("output-format")
            .long("--output-format")
            .takes_value(true)
            .possible_values(&["csv", "jsonl", "json"])
            .default_value("csv")
            .help("Output format."))
        .arg(
            Arg::with_name("no-headers")
                .long("--no-headers")
                .takes_value(false)
                .help("Don't print headers when using CSV as the output format"))
        .arg(
            Arg::with_name("normalize")
                .long("--normalize")
                .takes_value(false)
                .help("Normalize the result to the most important fields"))
        .get_matches()
}

fn output_data_csv(data: HashMap<String, String>) -> String
{
    format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
    data.get("target_full_path").unwrap(),
    data.get("target_modification_time").unwrap(),
    data.get("target_access_time").unwrap(),
    data.get("target_creation_time").unwrap(),
    data.get("target_size").unwrap(),
    data.get("target_hostname").unwrap(),
    data.get("lnk_full_path").unwrap(),
    data.get("lnk_modification_time").unwrap(),
    data.get("lnk_access_time").unwrap(),
    data.get("lnk_creation_time").unwrap())
}

fn main() {
    let args = parse_cli_args();
    let output_format = OutputFormat::from_str(args.value_of("output-format").unwrap());
    let output_to = args.value_of("output").unwrap();
    let normalize = match args.occurrences_of("normalize"){
        0 => false,
        _ => true
    };
    let mut output: Box<dyn Write> = match output_to {
        "stdout" => Box::new(io::stdout()),
        _ => Box::new(File::create(output_to).unwrap())
    };

    if args.occurrences_of("no-headers") == 0 {
        match output_format {
            OutputFormat::CSV => {
                output.write(r#""target_full_path","target_modification_time","target_access_time","target_creation_time","target_size","target_hostname","lnk_full_path","lnk_modification_time","lnk_access_time","lnk_creation_time""#.as_bytes()).expect("Error Writing Data !");
                output.write(b"\r\n").expect("Error Writing Data !");
            },
            _ => {}
        };
    }

    let mut lnk_file_paths = vec![
        "C:\\Windows\\SysWOW64\\config\\systemprofile\\AppData\\Roaming\\Microsoft\\Windows\\**\\*.lnk",
        "C:\\Windows\\System32\\config\\systemprofile\\AppData\\Roaming\\Microsoft\\Windows\\**\\*.lnk",
        "C:\\Users\\*\\AppData\\Roaming\\Microsoft\\Windows\\Recent\\*.lnk"
    ];
    if args.occurrences_of("PATH") > 0 {
        // override lnk_file_paths if the argument -p/--path is specified.
        lnk_file_paths = args.values_of("PATH").unwrap().collect();
    }


    #[derive(Debug, Serialize)]
    #[serde(untagged)]
    enum JsonRecord {
        Raw(LNKParser),
        Normalize(HashMap<String,String>)
    }
    let mut json_list = vec![];
    for dir in lnk_file_paths {
        for entry in glob(dir).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let full_path = path.as_path().to_str().unwrap();
                    match LNKParser::from_path(full_path) {
                        Ok(parsed) => {
                            match output_format {
                                OutputFormat::JSONL => {
                                    let json_data;
                                    if normalize {
                                        json_data = serde_json::to_string(&parsed.normalize()).unwrap();
                                    }
                                    else {
                                        json_data = serde_json::to_string(&parsed).unwrap();
                                    }
                                    output.write(json_data.as_bytes()).expect("Error Writing Data !");
                                    output.write(b"\r\n").expect("Error Writing Data !");
                                },
                                OutputFormat::JSON => {
                                    if normalize {
                                        json_list.push(JsonRecord::Normalize(parsed.normalize()));
                                    }
                                    else {
                                        json_list.push(JsonRecord::Raw(parsed));
                                    }
                                }
                                OutputFormat::CSV => {
                                    output.write(output_data_csv(parsed.normalize()).as_bytes()).expect("Error Writing Data !");
                                    output.write(b"\r\n").expect("Error Writing Data !");
                                }
                            }
                        },
                        Err(e) => {eprintln!("Did not parse '{}' correctly. ERROR : '{}'", full_path, e);}
                    };
                },
                Err(e) => eprintln!("{:?}", e)
            }
        }
    }
    if let OutputFormat::JSON = output_format {
        let json_data = serde_json::to_string(&json_list).unwrap();
        output.write(json_data.as_bytes()).expect("Error Writing Data !");
    }
}