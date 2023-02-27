#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
use dwy_vest::{bin, App};
use rocket::{
    data::{Data, ToByteUnit},
    fs::NamedFile,
    serde::{json::Json, Deserialize},
    tokio,
};
use std::path::{Path, PathBuf};

#[get(
    "/<user>/<id>/data/json/<ftl_files..>",
    format = "application/json",
    data = "<user>"
)]
async fn fetch_ftl_files(user: UserModel, id: u8, ftl_files: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(ftl_files))
        .await
        .ok()
}

#[post("/<user>/<id>/data/video/<video_datum..>", data = "<video_datum>")]
async fn fetch_video_data(user: UserModel, id: u8, video_datum: Data<'_>) -> std::io::Result<()> {
    // Stream at most 512KiB all of the body data to stdout.
    video_datum.open(512.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;

    Ok(())
}