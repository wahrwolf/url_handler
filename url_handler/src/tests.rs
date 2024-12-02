use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

mod file;
mod formats;
mod url_handler;

use super::format_handler::*;
use super::protocol_handler::*;

use super::*;

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
