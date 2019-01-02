#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs;
use std::fs::DirEntry;

const VID_EXTENSIONS: [&str; 1] = ["mkv"];
const SUB_EXTENSIONS: [&str; 1] = ["srt"];


fn main() {
    //use glob
    let directory = fs::read_dir("/home/alejandro/Documents/DATA/series/The.Office Season 1-8 [720p] [HD]/The.Office - Season 4 [720p]").unwrap();

    let (vid, sub): (Vec<DirEntry>, Vec<DirEntry>) = directory
        .into_iter()
        .filter_map(|result| result.ok().and_then(to_episode))
        .partition(is_video); // can convert to episode above and take an episode in is_video

}

fn is_video(entry: &DirEntry) -> bool {
    has_extension(entry, &VID_EXTENSIONS)
}

fn has_extension(entry: &DirEntry, extensions: &[&str]) -> bool {
    entry.path().extension()
        .map_or(false, |ext| extensions.contains(&ext.to_str().unwrap()))
}

fn vid_or_sub_entry(entry: DirEntry) -> Option<DirEntry> {
    lazy_static! {
        static ref ALL_EXTENSIONS: Vec<&'static str> = VID_EXTENSIONS.iter().chain(SUB_EXTENSIONS.iter()).map(|&x| x).collect();
    }

    match  has_extension(&entry, &ALL_EXTENSIONS) {
        true =>Some(entry),
        false => None
    }
}

fn to_episode(entry: &DirEntry) -> Option<Episode> {
    let file_name = entry.file_name().to_str().unwrap();
    None
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Episode<'a> {
    season: u32,
    episode: u32,
    entry: &'a DirEntry //Use str
}
