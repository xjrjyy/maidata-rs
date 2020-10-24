pub mod container;
pub mod insn;
pub mod materialize;

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("usage: $0 <path/to/maidata.txt>");
    let content = read_file(&filename);
    let maidata = container::lex_maidata(&content);

    println!("title = {}", maidata.title());
    println!("artist = {}", maidata.artist());

    for diff in maidata.iter_difficulties() {
        use std::borrow::Cow;

        println!();
        println!("difficulty {:?}", diff.difficulty());
        println!(
            "  level {}",
            diff.level()
                .map_or(Cow::Borrowed("<not set>"), |x| Cow::Owned(format!("{}", x)))
        );
        println!(
            "  offset {}",
            diff.offset()
                .map_or(Cow::Borrowed("<not set>"), |x| Cow::Owned(format!("{}", x)))
        );
        println!("  designer {}", diff.designer().unwrap_or("<not set>"));
        println!(
            "  static message {}",
            diff.single_message().unwrap_or("<not set>")
        );

        let mut mcx = materialize::context::MaterializationContext::with_offset(0.0);
        let notes = mcx.materialize_insns(diff.iter_insns());
        println!("  <{} notes>", notes.len());
    }
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}

pub(crate) type NomSpan<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Difficulty {
    /// The EASY difficulty.
    Easy = 1,
    /// The BASIC difficulty.
    Basic = 2,
    /// The ADVANCED difficulty.
    Advanced = 3,
    /// The EXPERT difficulty.
    Expert = 4,
    /// The MASTER difficulty.
    Master = 5,
    /// The Re:MASTER difficulty.
    ReMaster = 6,
    /// The ORIGINAL difficulty, previously called mai:EDIT in 2simai.
    Original = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Level {
    /// The "Lv.X" form.
    Normal(u8),
    /// The "Lv.X+" form.
    Plus(u8),
    /// The special "Lv.<any char>" form.
    Char(char),
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Level::*;
        match self {
            Normal(lv) => write!(f, "{}", lv)?,
            Plus(lv) => write!(f, "{}+", lv)?,
            Char(lv) => write!(f, "{}", lv)?,
        }

        Ok(())
    }
}
