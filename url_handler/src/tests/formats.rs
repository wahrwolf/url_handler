use super::*;

#[test]
fn struct_can_be_stored_in_toml() {
    let good_record = TestStruct::build_foo();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "toml", &handlers)
        .expect("Could not serialize record");
    let candidate: TestStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: TestStruct = build_record_from_string_with_extension(&string, "toml", &handlers)
        .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = TestStruct::build_bar();
    assert_ne!(bad_record, candidate);
}

#[test]
fn struct_can_be_stored_in_json() {
    let good_record = TestStruct::build_foo();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "json", &handlers)
        .expect("Could not serialize record");
    let candidate: TestStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: TestStruct = build_record_from_string_with_extension(&string, "json", &handlers)
        .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = TestStruct::build_bar();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_without_items_can_be_stored_in_toml() {
    let good_record = NestedStruct::build_struct_without_items();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "toml", &handlers)
        .expect("Could not serialize record");
    let candidate: NestedStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: NestedStruct =
        build_record_from_string_with_extension(&string, "toml", &handlers)
            .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = NestedStruct::build_struct_with_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_without_items_can_be_stored_in_json() {
    let good_record = NestedStruct::build_struct_without_items();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "json", &handlers)
        .expect("Could not serialize record");
    let candidate: NestedStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: NestedStruct =
        build_record_from_string_with_extension(&string, "json", &handlers)
            .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = NestedStruct::build_struct_with_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_with_items_can_be_stored_in_toml() {
    let good_record = NestedStruct::build_struct_with_items();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "toml", &handlers)
        .expect("Could not serialize record");
    let candidate: NestedStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: NestedStruct =
        build_record_from_string_with_extension(&string, "toml", &handlers)
            .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = NestedStruct::build_struct_without_items();
    assert_ne!(bad_record, candidate);
}

#[test]
fn nested_struct_with_items_can_be_stored_in_json() {
    let good_record = NestedStruct::build_struct_with_items();
    let handlers = FormatHandlerRegistry::new();
    let string = build_string_from_record_with_extension(&good_record, "json", &handlers)
        .expect("Could not serialize record");
    let candidate: NestedStruct =
        build_record_from_string(&string, &handlers).expect("Could not deserialize record");
    assert_eq!(good_record, candidate.clone());

    let duplicate: NestedStruct =
        build_record_from_string_with_extension(&string, "json", &handlers)
            .expect("Could not deserialize record");
    assert_eq!(duplicate, candidate.clone());

    let bad_record = NestedStruct::build_struct_without_items();
    assert_ne!(bad_record, candidate);
}
