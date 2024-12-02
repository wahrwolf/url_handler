use super::*;
use crate::try_build_url_from_path_buf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tempfile::TempDir;

#[derive(Default, Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize)]
struct TestStruct {
    last_updated: DateTime<Utc>,
    id: u32,
    name: String,
    is_pretty: bool,
}

impl TestStruct {
    pub fn build_foo() -> TestStruct {
        TestStruct {
            last_updated: Utc::now(),
            id: 1,
            name: String::from("Foo"),
            is_pretty: true,
        }
    }
    pub fn build_bar() -> TestStruct {
        TestStruct {
            last_updated: Utc::now(),
            id: 1,
            name: String::from("Bar"),
            is_pretty: false,
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
struct NestedStruct {
    map: HashMap<String, TestStruct>,
    set: HashSet<TestStruct>,
}

impl NestedStruct {
    pub fn build_struct_with_items() -> NestedStruct {
        NestedStruct {
            map: HashMap::from([("Foo".to_string(), TestStruct::build_foo())]),
            set: HashSet::from([TestStruct::build_bar()]),
        }
    }
    pub fn build_struct_without_items() -> NestedStruct {
        NestedStruct {
            map: HashMap::default(),
            set: HashSet::default(),
        }
    }
}

#[test]
fn struct_can_be_stored_in_toml() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.toml");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = TestStruct::build_foo();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: TestStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = TestStruct::build_bar();
    assert_ne!(bad_record, candidate);
}

#[test]
fn struct_can_be_stored_in_json() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.json");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = TestStruct::build_foo();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: TestStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = TestStruct::build_bar();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_without_items_can_be_stored_in_toml() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.toml");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = NestedStruct::build_struct_without_items();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: NestedStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = NestedStruct::build_struct_with_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_without_items_can_be_stored_in_json() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.json");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = NestedStruct::build_struct_without_items();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: NestedStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = NestedStruct::build_struct_with_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_with_items_can_be_stored_in_json() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.json");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = NestedStruct::build_struct_with_items();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: NestedStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = NestedStruct::build_struct_without_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_with_items_can_be_stored_in_toml() {
    let tmp_dir: TempDir = TempDir::new().expect("Could not create TempDir");
    let target_file = tmp_dir.path().join("test.toml");
    let url = try_build_url_from_path_buf(&target_file).expect("Could not build url");
    let format_handlers = FormatHandlerRegistry::default();
    let protocol_handlers = ProtocolHandlerRegistry::default();
    let good_record = NestedStruct::build_struct_with_items();
    push_record_to_url(&url, &good_record, &protocol_handlers, &format_handlers)
        .expect("Could not push record");

    let candidate: NestedStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)
        .expect("Could not parse record");

    assert_eq!(good_record, candidate.clone());

    let bad_record = NestedStruct::build_struct_without_items();
    assert_ne!(bad_record, candidate);
}
