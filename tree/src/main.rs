use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

// Better to have these hard-coded
const SYMB_TFT: &str = "└── ";
const SYMB_TTT: &str = "├── ";

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
    filler_previous: &str,  // Filler string (prefix of the file/directory name)
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

        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (idx, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last: bool = entries.len() - idx - 1 == 0;

            let mut filler_next = String::new();
            let mut filler_present = String::new();

            match is_last {
                true => filler_present.push_str(SYMB_TFT),
                false => filler_present.push_str(SYMB_TTT),
            }

            filler_next.push_str(&filler_previous);
            filler_next.push_str(&filler_present);

            if path.is_dir() || (path.is_file() && !directories_only) {
                println!(
                    "{}{}{}",
                    &filler_previous,
                    &filler_present,
                    &path.file_name().unwrap().to_str().unwrap()
                );
            }

            if path.is_dir() {
                walk(
                    &path,
                    max_depth,
                    depth + 1,
                    &filler_next
                        .replace("─", " ")
                        .replace("├", "│")
                        .replace("└", " "),
                    all,
                    directories_only,
                )?;
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
