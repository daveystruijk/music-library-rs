extern crate clap;
extern crate glob;
extern crate id3;
extern crate serde;
extern crate serde_json;
extern crate image;
extern crate termimage;

use termimage::*;
use std::fs;
use std::io::stdout;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::json;
use clap::{Arg, App, SubCommand};
use glob::glob;
use id3::Tag;
use image::{ImageFormat, DynamicImage, FilterType};

const LIBRARY_FILENAME: &str = "library.json";

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Library {
    tracks: Vec<Track>,
}

fn main() {
    let matches = App::new("Music Library")
        .author("Davey Struijk <mail@daveystruijk.com>")
        .about("A music library management tool.")
		.subcommand(SubCommand::with_name("init")
            .arg(Arg::with_name("debug")
                 .help("print debug information verbosely")))
		.subcommand(SubCommand::with_name("scan")
            .arg(Arg::with_name("debug")
                 .help("print debug information verbosely")))
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => init_library(),
        Some("scan") => scan_current_directory(),
        None => println!("No subcommand was used"),
        _ => println!("Unimplemented"),
    }

}

fn init_library() {
    let library = json!({
        "tracks": [],
    });
    let library_json_str = serde_json::to_string(&library)
        .expect("Failed to parse library");
    fs::write(LIBRARY_FILENAME, library_json_str)
        .expect("Failed to write to library");
    println!("Initialized empty library in {}", LIBRARY_FILENAME);
}

fn load_library() -> serde_json::Result<Library> {
    let library_file = fs::File::open(LIBRARY_FILENAME)
        .expect("File not found");
    let library: Library = serde_json::from_reader(library_file)
        .expect("Error while reading json");
    return Ok(library);
}

fn scan_current_directory() {
    let library: Library = load_library().expect("Failed to load library");
    println!("{:?}", library);

    for trackpath in glob("**/*.[mM][pP]3").unwrap().filter_map(Result::ok) {
        scan_file(trackpath);
        return;
    }
}

fn print_album_art(picture: &id3::frame::Picture, size: u32) {
    let format = match picture.mime_type.as_str() {
        "image/png" => ImageFormat::PNG,
        "image/jpg" => ImageFormat::JPEG,
        _ => panic!("Wrong image format"),
    };

    let img = image::load(std::io::Cursor::new(&picture.data), format)
        .expect("Failed to load image from ID3 tag");
    let resized = img.resize(size, size, FilterType::Nearest);

    ops::write_ansi_truecolor(&mut stdout(), &resized);
}

fn scan_file(trackpath: std::path::PathBuf) {
    let tag = Tag::read_from_path(&trackpath).unwrap();
    let picture = tag.get("APIC").unwrap().content().picture().unwrap();
    print_album_art(picture, 25);

    // let track = Track {
    //     path: trackpath.to_str().unwrap().to_string(),
    // };
    // println!("{:?}", tag);
}
