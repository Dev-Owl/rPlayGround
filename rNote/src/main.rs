extern crate rustc_serialize;
extern crate time;
extern crate docopt;

mod setting;
mod notes;
mod setup;

use notes::Note;
use docopt::Docopt;
use std::env;

//TODO: Path handling is not correct change to PathBuf ??
//TODO: Add function to check cmd arguments
//TODO: Add function to search for notes
//TODO: Add function to print notes to screen (inside note obj)

const USAGE: &'static str="
rNote.

Usage:
    rNote new --ti=<title> [--ta=<tag>... --p=<project> --d=<description>]
    rNote update <id> [--ti=<title>  --ta=<tag>... --p=<project> --d=<description> --e]
    rNote search [--i=<id> --ti=<title> --ta=<tag> --p=<project> --d=<description> --a --c]
    rNote start --i=<id>
    rNote stop --i=<id>
    rNote done --i=<id>
    rNote delete --i=<id>
    rNote --h | --help
Options:
      -h --help          Show this screen.
      --ti=<title>       The title of the note
      --ta=<tag>         Tag of a note,set or search in it.
      --p=<project>      Project of a note.
      --d=<description>  Description of a note.
      --i=<id>           The id of a note.
      --a                Show all notes, by default done notes are hidden
      --c                Just count no reuslt
      --e                Erase given elements
";

fn main() {


   //Setup env for application
   setup::env_check();
   //Get connfiguration
   let settings = setting::get_config();let settings = setting::get_config();
   //Parse cmd arguments
   print_note();
}

fn print_note()
{
    let settings = setting::get_config();let settings = setting::get_config();
    let new_id: u32 = Note::new_id( &settings.get_default("data","data"),
                                        settings.get_default("id_offset","500").parse::<u32>().unwrap());

    let mut my_note = Note::new(new_id,"Owls everywhere".to_string());
    my_note.add_tag("Testing".to_string());
    my_note.add_tag("FuBar".to_string());
    my_note.add_tag("Wurst".to_string());
    my_note.set_text("This is my very long description for a very short and easy test task :)");
    println!("{}", my_note);

}

#[test]
fn test_cmd_new()
{
    let argv = || vec!["rNote","new","--ti=FuBar","--ta=su1", "--ta=su2", "--ta=su3", "--p=fupro","--d=lorodo aroparo"];
    let args = Docopt::new(USAGE)
                  .and_then(|d| d.argv(argv().into_iter()).parse())
                  .unwrap_or_else(|e| e.exit());
    assert_eq!(args.get_str("--ti"),"FuBar");
    assert_eq!(args.get_count("new"),1);
    assert_eq!(args.get_vec("--ta"), vec!["su1", "su2", "su3"]);
    assert_eq!(args.get_str("--p"), "fupro");
    assert_eq!(args.get_str("--d"), "lorodo aroparo");
}

#[test]
fn test_cmd_update()
{
    let argv = || vec!["rNote","update","123","--ti=FuBar","--e"];
    let args = Docopt::new(USAGE)
                  .and_then(|d| d.argv(argv().into_iter()).parse())
                  .unwrap_or_else(|e| e.exit());
    assert_eq!(args.get_count("update"),1);
    assert_eq!(args.get_str("<id>"),"123");
    assert_eq!(args.get_str("--ti"),"FuBar");
    assert_eq!(args.get_bool("--e"),true);
}
