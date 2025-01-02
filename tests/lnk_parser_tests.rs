use glob::glob;
use lnk_parser::LNKParser;
use std::path::PathBuf;

#[cfg(test)]
#[test]
fn test_win7() {
    let path = ["samples", "WIN7", "*", "*.lnk"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_win10() {
    let path = ["samples", "WIN10", "*", "*.lnk"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_ws12r2() {
    let path = ["samples", "WS12R2", "*", "*.lnk"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_ws19() {
    let path = ["samples", "WS19", "*", "*.lnk"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_other() {
    let path = ["samples", "other", "*"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_mal() {
    let path = ["samples", "mal", "*"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    for entry in glob(path.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let full_path = path.to_str().unwrap().to_string();
                println!("{}", full_path);
                println!("{:?}", LNKParser::from_path(&full_path).unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
