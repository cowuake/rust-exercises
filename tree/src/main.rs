use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

// Better to have these hard-coded
const SYMB_TFT: &str = "└── ";
const SYMB_TTT: &str = "├── ";
const SYMB_FFF: &str = "    ";
const SYMB_TTF: &str = "│   ";

#[derive(Debug, StructOpt)]
#[structopt(name = "tree", about = "Basic tree implementation in Rust.")]
struct Opt {
    /// Include hidden entries
    #[structopt(short, long)]
    all: bool,

    /// Root directory (starting point)
    #[structopt(name = "DIR", default_value = ".", parse(from_os_str))]
    dir: PathBuf,

    /// List directories only
    #[structopt(short)]
    directories_only: bool,

    /// Max directory tree level
    #[structopt(short, long, default_value = "999")]
    max_depth: usize,
}

fn walk(
    dir: &Path,             // Path to be used as the root
    max_depth: usize,       // Maximum depth level to be used
    depth: usize,           // Present depth level
    filler: &str,           // Filler string (prefix of the file/directory name)
    all: bool,              // True if hidden files have to be included
    directories_only: bool, // True if only directory have to be listed
) -> Result<()> {
    if dir.is_dir() {
        if max_depth - depth == 0 {
            return Ok(());
        }

        let root = fs::read_dir(dir);

        let mut entries: Vec<fs::DirEntry> = vec![];

        for e in root.unwrap() {
            if e.as_ref().unwrap().path().is_dir()
                || (e.as_ref().unwrap().path().is_file() && !directories_only)
            {
                if all {
                    // Show all files and directories, included hidden ones
                    entries.push(e.unwrap());
                } else {
                    // Ignore hidden files and directories
                    if !e
                        .as_ref()
                        .unwrap()
                        .file_name()
                        .to_str()
                        .unwrap()
                        .starts_with(".")
                    {
                        entries.push(e.unwrap());
                    }
                }
            }
        }

        //if all { // Show all files and directories, included hidden ones
        //    for e in root.unwrap() {
        //	entries.push(e.unwrap());
        //    }
        //} else { // Ignore hidden files and directories
        //    for e in root.unwrap() {
        //	if ! e.as_ref().unwrap().file_name().to_str().unwrap().starts_with(".") {
        //	    entries.push(e.unwrap());
        //	}
        //    }
        //}

        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (idx, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let mut filler_new = String::new();

            if entries.len() - idx == 1 {
                // Second-last entry
                if path.is_dir() || (path.is_file() && !directories_only) {
                    println!(
                        "{}{}{}",
                        &filler,
                        SYMB_TFT,
                        &path.file_name().unwrap().to_str().unwrap()
                    );
                }
                if path.is_dir() {
                    filler_new.push_str(SYMB_FFF);
                    walk(
                        &path,
                        max_depth,
                        depth + 1,
                        &filler_new,
                        all,
                        directories_only,
                    )?;
                }
            } else {
                // All entries except the second-last
                if path.is_dir() || (path.is_file() && !directories_only) {
                    println!(
                        "{}{}{}",
                        &filler,
                        SYMB_TTT,
                        &path.file_name().unwrap().to_str().unwrap()
                    );
                }
                if path.is_dir() {
                    filler_new.push_str(SYMB_TTF);
                    walk(
                        &path,
                        max_depth,
                        depth + 1,
                        &filler_new,
                        all,
                        directories_only,
                    )?;
                }
            }
        }

        Ok(())
    } else {
        Ok(())
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match fs::read_dir(&opt.dir) {
        Ok(_) => println!("{:?}", opt.dir),
        Err(_) => {
            println!("Error! {:?} is not a valid directory.", opt.dir);
            process::exit(1);
        }
    }

    walk(
        &opt.dir,
        opt.max_depth,
        0,
        "",
        opt.all,
        opt.directories_only,
    )
}
