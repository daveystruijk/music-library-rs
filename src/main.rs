extern crate glob;
extern crate id3;

use id3::Tag;
use glob::glob;

struct Track {
    path: std::path::PathBuf
}

fn main() {
    for trackpath in glob("**/*.[mM][pP]3").unwrap().filter_map(Result::ok) {
        let mut track = Track {
            path: trackpath
        };
        analyze(track);
    }
}

fn analyze(track: Track) {
    println!("{:?}", track.path);
}
