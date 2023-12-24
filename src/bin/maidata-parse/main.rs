use walkdir::WalkDir;

fn main() {
    let dir = std::env::args().nth(1).expect("usage: $0 <path/to/charts>");

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        if entry.file_name() == "maidata.txt" {
            println!("{:?}", entry.path());
            parse_maidata(entry.path());
        }
    }
}

fn parse_maidata<P: AsRef<std::path::Path>>(path: P) {
    let content = read_file(path);
    let maidata = maidata::container::lex_maidata(&content);

    for diff in maidata.iter_difficulties() {
        println!("{} insns", diff.iter_insns().count());

        // for insn in diff.iter_insns() {
        //     println!("{:?}", insn);
        // }
    }
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
