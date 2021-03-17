#[macro_use] extern crate log;
#[macro_use] extern crate cli_log;

use {
    locmess::*,
    std::{
        env,
        path::PathBuf,
    },
};

fn main() -> anyhow::Result<()> {
    init_cli_log!();
    let args: Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("locmess {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let printer = Printer::new(&args);
    let mut root = args.file
        .as_ref()
        .map_or(env::current_dir()?, PathBuf::from);
    if root.is_relative() {
        root = env::current_dir()?.join(root);
    }
    root = root.canonicalize()?;
    let file_finder = FileFinder::new(root);
    let mut files = if let Some(ext) = args.file_type {
        file_finder.source_files(Some(&ext))
    } else {
        file_finder.main_type_files()
    }?;
    let mut total_histo = Histogram::default();
    let mut paths_over = Vec::new();
    for path in files.drain(..) {
        let mut histo = Histogram::default();
        match read(&path, &mut histo) {
            Ok(max_len) => {
                if let Some(over) = args.over {
                    if max_len > over {
                        paths_over.push(path)
                    }
                }
                total_histo += histo;
            }
            Err(e) => {
                eprintln!("Ignoring {:?} : {}", &path, e);
            }
        }
    }
    total_histo.print(&printer);
    if let Some(over) = args.over {
        if paths_over.is_empty() {
            println!("No file with a line over {} chars", over);
        } else {
            if paths_over.len() == 1 {
                println!("1 file with at least one line over {} chars:", over);
            } else {
                println!("{} files with at least one line over {} chars:", paths_over.len(), over);
            }
            for path in paths_over {
                println!("{}", path.to_string_lossy());
            }
        }
    }
    info!("bye");
    Ok(())
}
