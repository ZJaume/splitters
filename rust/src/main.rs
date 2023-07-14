use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::env;
use srx::SRX;
use clap::Parser;
use pyo3::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ISO-639-1, 2 char language code
    #[clap(short, long, default_value="en")]
    language: String,
    #[clap(short, long)]
    input: String,
    #[clap(short, long)]
    output: String,
    #[clap(short, long, default_value="")]
    srxfile: String,
    #[clap(short, long)]
    verbose: bool,
}

// Call python interpreter and obtain python path of our module
fn pythonpath() -> PyResult<String> {
    let mut path = String::new();
    Python::with_gil(|py| {
        // Instead of hardcoding the module name, obtain it from the crate name at compile time
        let module = PyModule::import(py, env!("CARGO_PKG_NAME"))?;
        let paths: Vec<&str> = module
            .getattr("__path__")?
            .extract()?;
        // __path__ attribute returns a list of paths, return first
        path.push_str(paths[0]);
        Ok(path)
    })
}

// Choose best rule for each language according to benchmark:
// https://docs.google.com/spreadsheets/d/1mGJ9MSyMlsK0EUDRC2J50uxApiti3ggnlrzAWn8rkMg/edit?usp=sharing
fn choose_srx(lang: &str) -> &str {
    let lt_langs = ["da", "de", "en", "fr", "nl", "pl", "pt", "ro", "sk", "sr", "tr", "uk"];
    let omegat_langs = ["bg", "cs", "es", "me", "mk", "mt", "sl", "sq", "sv"];
    let ptdr_langs = ["el", "et", "fi", "hr", "hu", "is", "lt", "lv"];
    let nonaggr_langs = ["it", "nb", "nn"];

    if lt_langs.iter().any(|&x| x == lang) {
        return "language_tools.segment.srx";
    } else if omegat_langs.iter().any(|&x| x == lang) {
        return "OmegaT.srx";
    } else if ptdr_langs.iter().any(|&x| x == lang) {
        return "PTDR.srx";
    } else if nonaggr_langs.iter().any(|&x| x == lang) {
        return "NonAggresive.srx";
    } else{
        return "default.srx";
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut srxfile = String::new();

    // If srx file is not provided, try to load one of the packaged srx files
    if args.srxfile.is_empty() {
        srxfile.push_str(
            format!(
                "{}/data/{}",
                // Obtain the path of the python package, where srx files are
                pythonpath().unwrap().as_str(),
                choose_srx(args.language.as_str()),
            ).as_str()
        );
        if !Path::new(&srxfile).exists() {
            panic!(
                "SRX file could not be found in '{}'. Please provide one with '-s'.",
                srxfile,
            );
        }
    } else {
        srxfile.push_str(args.srxfile.as_str());
    }

    // Prepare output file to be written segment by segment
    let file = File::create(args.output)?;
    let mut file = LineWriter::new(file);
    // Load SRX rules from file
    let srx =
        SRX::from_str(&read_to_string(srxfile.clone()).expect("rules file exists"))
            .expect("srx rule file is valid");
    if args.verbose {
        eprintln!("Loaded SRX file: {}", srxfile);
        eprintln!("SRX rules errors while parsing with regex, by language:");
        for (k, v) in srx.errors(){
            eprintln!("{:?}:", k);
            for i in v{
                eprintln!("{}", i);
            }
        }
    }
    let rules = srx.language_rules(args.language);
    if args.verbose {
        eprintln!("Using these rules for the selected language: {:?}", rules);
    }

    // Read each input file line (it could be a whole document)
    if let Ok(lines) = read_lines(args.input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(doc) = line {
                // Split the content using the SRX segmenter and write each segment to the output
                for splittedline in rules.split(&doc).collect::<Vec<_>>(){
                    file.write(splittedline.as_bytes())?;
                    file.write(b"\n")?;
                }
            }
        }
    }

    file.flush()?;
    Ok(())

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

