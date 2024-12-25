# About
With this library you can create, update and delete strings and records from a URL.

# Example Usage
```rust

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use url_handler::{FormatHandlerRegistry, ProtocolHandlerRegistry};

#[derive(Default, Clone, Eq, Hash, Debug, Serialize, Deserialize)]
struct TestStruct {
    last_updated: DateTime<Utc>,
    id: u32,
    name: String,
    is_pretty: bool,
}

let protocol_handlers = ProtocolHandlerRegistry::default();
let format_handlers = FormatHandlerRegistry::default();

let candidate: NestedStruct = build_record_from_url(&url, &protocol_handlers, &format_handlers)?;
```


# Supported Formats
- TOML
- JSON

# Supported Protocols
- local files (file://)
- remote files (scp://)
- http

# Next Steps
- more formats
- more protocols
- more tests?

