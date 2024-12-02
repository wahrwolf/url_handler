use super::*;
use crate::try_build_url_from_path_buf;
use std::fs::{read_to_string, write};
use tempfile::TempDir;

#[test]
fn string_can_be_pushed_to_file() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test_push_record");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let payload: String = "Foobar".to_string();

    let handler = FileProtocolHandler::default();
    handler
        .push_string_to_url(&url, &payload)
        .expect("Could not push record");

    let record: String =
        read_to_string(target_file.clone()).expect("Could not read string from file");
    assert_eq!(payload, record);
}

#[test]
fn string_can_be_fetched_from_file() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test_fetch_record");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let payload: String = "Foobar".to_string();

    write(&target_file, &payload).expect("Could not write payload to file");

    let handler = FileProtocolHandler::default();
    let record = handler
        .fetch_string_from_url(&url)
        .expect("Could not fetch record");

    assert_eq!(Some(payload), record);
}

#[test]
fn string_can_be_fetched_after_pushing() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test_fetch_record");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let payload: String = "Foobar".to_string();

    let handler = FileProtocolHandler::default();
    handler
        .push_string_to_url(&url, &payload)
        .expect("Could not push record");

    let record = handler
        .fetch_string_from_url(&url)
        .expect("Could not fetch record");

    assert_eq!(Some(payload), record);
}
