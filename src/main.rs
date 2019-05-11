#![allow(proc_macro_derive_resolution_fallback)]
#![recursion_limit = "128"]
#[macro_use]
extern crate diesel;

mod adm;
mod env;
mod fetch_places;
mod models;
mod myexif;
mod photosdir;
mod pidfiles;
mod schema;
mod server;

use crate::adm::result::Error;
use crate::adm::stats::show_stats;
use crate::adm::{findphotos, makepublic, precache, storestatics, users};
use crate::env::{dburl, photos_dir};
use crate::models::Coord;
use crate::photosdir::PhotosDir;
use clap::{App, Arg, ArgMatches, SubCommand};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::process::exit;

fn main() {
    dotenv().ok();
    env_logger::init();
    let args = App::new("rphotos")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Command line interface for rphotos")
        .subcommand(
            SubCommand::with_name("findphotos")
                .about("Find new photos in the photo directory")
                .arg(Arg::with_name("BASE").multiple(true).help(
                    "Base directory to search in (relative to the \
                     image root).",
                )),
        ).subcommand(
            SubCommand::with_name("stats")
                .about("Show some statistics from the database"),
        ).subcommand(
            SubCommand::with_name("userlist").about("List existing users"),
        ).subcommand(
            SubCommand::with_name("userpass")
                .about("Set password for a (new or existing) user")
                .arg(
                    Arg::with_name("USER")
                        .required(true)
                        .help("Username to set password for"),
                ),
        ).subcommand(
            SubCommand::with_name("fetchplaces")
                .about("Get place tags for photos by looking up coordinates in OSM")
                .arg(
                    Arg::with_name("LIMIT")
                        .long("limit")
                        .short("l")
                        .takes_value(true)
                        .default_value("5")
                        .help("Max number of photos to use for --auto")
                ).arg(
                    Arg::with_name("AUTO")
                        .long("auto")
                        .help("Fetch data for photos with position but \
                               lacking places.")
                ).arg(
                    Arg::with_name("PHOTOS")
                        .required_unless("AUTO").multiple(true)
                        .help("Image ids to fetch place data for"),
                ),
        ).subcommand(
            SubCommand::with_name("makepublic")
                .about("make specific image(s) public")
                .arg(
                    Arg::with_name("LIST")
                        .long("list")
                        .short("l")
                        .takes_value(true)
                        .help("File listing image paths to make public"),
                ).arg(
                    Arg::with_name("IMAGE")
                        .required_unless("LIST")
                        .help("Image path to make public"),
                ).after_help(
                    "The image path(s) are relative to the image root.",
                ),
        ).subcommand(
            SubCommand::with_name("precache")
                .about("Make sure the photos has thumbnails stored in cache.")
                .arg(
                    Arg::with_name("MAXTIME")
                        .long("max-time")
                        .default_value("10")
                        .help("Max time (in seconds) to work")
                ).after_help(
                    "The time limit is checked after each stored image, \
                     so the command will complete in slightly more than \
                     the max time and one image will be processed even \
                     if the max time is zero."
                )
        ).subcommand(
            SubCommand::with_name("storestatics")
                .about("Store statics as files for a web server")
                .arg(
                    Arg::with_name("DIR")
                        .required(true)
                        .help("Directory to store the files in"),
                ),
        ).subcommand(
            SubCommand::with_name("runserver")
                .arg(
                    Arg::with_name("PIDFILE")
                        .long("pidfile")
                        .takes_value(true)
                        .help(
                            "Write (and read, if --replace) a pid file with \
                             the name given as <PIDFILE>.",
                        ),
                ).arg(Arg::with_name("REPLACE").long("replace").help(
                    "Kill old server (identified by pid file) before running",
                )),
        ).get_matches();

    match run(&args) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}

fn run(args: &ArgMatches) -> Result<(), Error> {
    match args.subcommand() {
        ("findphotos", Some(args)) => {
            let pd = PhotosDir::new(photos_dir());
            let db = get_db()?;
            if let Some(bases) = args.values_of("BASE") {
                for base in bases {
                    findphotos::crawl(&db, &pd, Path::new(&base)).map_err(
                        |e| {
                            Error::Other(format!(
                                "Failed to crawl {}: {}",
                                base, e,
                            ))
                        },
                    )?;
                }
            } else {
                findphotos::crawl(&db, &pd, Path::new("")).map_err(|e| {
                    Error::Other(format!("Failed to crawl: {}", e))
                })?;
            }
            Ok(())
        }
        ("makepublic", Some(args)) => {
            let db = get_db()?;
            match args.value_of("LIST") {
                Some("-") => {
                    let list = io::stdin();
                    makepublic::by_file_list(&db, list.lock())?;
                    Ok(())
                }
                Some(f) => {
                    let list = File::open(f)?;
                    let list = BufReader::new(list);
                    makepublic::by_file_list(&db, list)
                }
                None => makepublic::one(&db, args.value_of("IMAGE").unwrap()),
            }
        }
        ("stats", Some(_args)) => show_stats(&get_db()?),
        ("userlist", Some(_args)) => users::list(&get_db()?),
        ("fetchplaces", Some(args)) => {
            let db = get_db()?;
            if args.is_present("AUTO") {
                let limit = args.value_of("LIMIT").unwrap().parse()?;
                println!("Should find {} photos to fetch places for", limit);
                use crate::schema::photo_places::dsl as place;
                use crate::schema::positions::dsl as pos;
                let result = pos::positions
                    .select((pos::photo_id, (pos::latitude, pos::longitude)))
                    .filter(pos::photo_id.ne_all(
                        place::photo_places.select(place::photo_id).distinct(),
                    ))
                    .order(pos::photo_id.desc())
                    .limit(limit)
                    .load::<(i32, Coord)>(&db)?;
                for (photo_id, coord) in result {
                    println!("Find places for #{}, {:?}", photo_id, coord);
                    fetch_places::update_image_places(&db, photo_id)?;
                }
            } else {
                for photo in args.values_of("PHOTOS").unwrap() {
                    fetch_places::update_image_places(&db, photo.parse()?)?;
                }
            }
            Ok(())
        }
        ("userpass", Some(args)) => {
            users::passwd(&get_db()?, args.value_of("USER").unwrap())
        }
        ("precache", Some(args)) => precache::precache(
            &get_db()?,
            &PhotosDir::new(photos_dir()),
            args.value_of("MAXTIME").unwrap().parse()?,
        ),
        ("storestatics", Some(args)) => {
            storestatics::to_dir(args.value_of("DIR").unwrap())
        }
        ("runserver", Some(args)) => server::run(args),
        _ => {
            println!("No subcommand given.\n\n{}", args.usage());
            Ok(())
        }
    }
}

fn get_db() -> Result<PgConnection, ConnectionError> {
    PgConnection::establish(&dburl())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
