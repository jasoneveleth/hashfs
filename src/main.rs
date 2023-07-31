use std::fs;
use std::path::PathBuf;
use sha2::Digest;
use anyhow::Result;
use anyhow::bail;
use std::env;
use sha2::Sha256;
use hex;
use std::borrow::Borrow;
use std::time::Instant;

fn hash(bytes: &[u8]) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hex::encode(hasher.finalize()))
}

fn bold(s: &str) -> String {
    "<strong>".to_string() + s + "</strong>"
}

fn visit(dir: &PathBuf, mut last_log: Instant) -> Result<String> {
    let mut contents = String::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        if last_log.elapsed().as_millis() > 1000 {
            println!("current entry: {}", entry.path().to_string_lossy());
            last_log = Instant::now();
        }

        let metadata = entry.metadata()?;
        if !metadata.is_dir() {
            let name = entry.path();
            // remove this line to make it the full pathname
            let name = name.file_name().expect("no filename");

            contents += &bold(name.to_string_lossy().borrow());
            contents += " ";
            contents += &hash(&fs::read(entry.path())?)?;
            contents += "</br>";
        } else {
            contents += &visit(&entry.path(), last_log)?;
        }
    }

    let contents_hash = &hash(contents.as_bytes())?;

    let mut ret = "<details><summary>".to_string();
    let name = dir.file_name().unwrap_or(dir.as_os_str());
    ret += &bold(name.to_string_lossy().borrow());
    ret += " ";
    ret += &contents_hash;
    ret += "</summary>";
    ret += &contents;

    Ok(ret + "</details>")
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Too few args");
    }
    let dir = &args[1];
    let html = visit(&PathBuf::from(dir), Instant::now())?;

    let data = "<html><body>".to_string() + &html + "<style>:root{--l: 25px}details > details{margin-left: var(--l);}summary{margin-left: calc(0px - var(--l))}html{font-family: monospace;margin-left: var(--l);}</style></body></html>";
    let output_file = "/tmp/dir2.html";
    fs::write(output_file, data).expect("Something is really wrong if /tmp is broken");
    println!("successfully written to {}!", output_file);
    Ok(())
}

