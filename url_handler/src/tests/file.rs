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

#[test]
fn empty_string_can_be_created() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test_empty_record");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let payload: String = "".to_string();

    let handler = FileProtocolHandler::default();
    handler
        .create_empty_string_on_url(&url)
        .expect("Could not create emtpy record");

    let record = handler
        .fetch_string_from_url(&url)
        .expect("Could not fetch record");

    assert_eq!(Some(payload), record);
}

#[test]
fn string_can_not_be_fetched_after_deleting() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test_fetch_record");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let payload: String = "Foobar".to_string();

    let handler = FileProtocolHandler::default();
    handler
        .push_string_to_url(&url, &payload)
        .expect("Could not push record");

    handler
        .delete_string_from_url(&url)
        .expect("Could not delete record");

    let record = handler.fetch_string_from_url(&url);

    assert!(record.is_err());
}

#[test]
fn dir_can_be_created_as_url_container() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_dir = tmp_dir.path().join("test_push_record");
    let url = try_build_url_from_path_buf(&target_dir).expect("Could not build url");

    assert!(!target_dir.exists());

    let handler = FileProtocolHandler::default();
    handler
        .create_url_container(&url)
        .expect("Could not push record");

    assert!(target_dir.exists());
}

#[test]
fn nested_url_containers_can_be_created() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let parent_dir = tmp_dir.path().join("test_parent_container");
    let target_dir = parent_dir.join("child");
    let target_url = try_build_url_from_path_buf(&target_dir).expect("Could not build url");

    assert!(!parent_dir.exists());
    assert!(!target_dir.exists());

    let handler = FileProtocolHandler::default();
    handler
        .create_url_container(&target_url)
        .expect("Could not push record");

    assert!(target_dir.exists());
    assert!(parent_dir.exists());
}

#[test]
fn urls_can_be_discovered_from_container() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");

    let container = tmp_dir.path().join("test_parent_container");
    let good_record = container.join("good_record");
    let bad_record = container.join("bad_record");

    let good_url = try_build_url_from_path_buf(&good_record).expect("Could not build url");
    let bad_url = try_build_url_from_path_buf(&bad_record).expect("Could not build url");
    let container_url = try_build_url_from_path_buf(&container).expect("Could not build url");

    assert!(!container.exists());
    assert!(!good_record.exists());
    assert!(!bad_record.exists());

    let handler = FileProtocolHandler::default();
    handler
        .create_empty_string_on_url(&good_url)
        .expect("Could not push record");

    assert!(container.exists());
    assert!(good_record.exists());
    assert!(!bad_record.exists());
    
    let candidate = handler
        .list_urls_in_url_container(&container_url)
        .expect("Could not list urls");

    assert!(candidate.contains(&good_url));
    assert!(!candidate.contains(&bad_url));
}
