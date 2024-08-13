use std::time::Instant;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).expect("usage: $0 <path/to/charts>");

    let start = Instant::now();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        if entry.file_name() == "maidata.txt" {
            // println!("{:?}", entry.path());
            parse_maidata(entry.path())?;
        }
    }

    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    Ok(())
}

fn parse_maidata<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let content = read_file(&path);
    let (_, state) = maidata::container::lex_maidata(&content);

    if state.has_messages() {
        println!("Path: {:?}", path.as_ref());
        for warning in &state.warnings {
            println!("Warning: {}", warning);
        }
        for error in &state.errors {
            println!("Error: {}", error);
        }
        println!();
    }

    // for diff in maidata.iter_difficulties() {
    //     println!("{} insns", diff.iter_insns().count());

    //     // for insn in diff.iter_insns() {
    //     //     println!("{:?}", insn);
    //     // }
    // }

    Ok(())
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
