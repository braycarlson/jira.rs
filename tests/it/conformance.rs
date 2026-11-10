use alloc::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const HEADER_XSRF_NAME: &str = "X-Atlassian-Token";
const HEADER_XSRF_SENT: &str = "request.header(\"X-Atlassian-Token\", \"no-check\");";

const TERMS_DECLARED: &str = "params";
const TERMS_SENT: &str = "request.query_pairs(self.properties)?;";

const METHODS: [(&str, &str); 5] =
    [("delete", "Delete"), ("get", "Get"), ("patch", "Patch"), ("post", "Post"), ("put", "Put")];

const MULTIPART: &str = "multipart/form-data";
const OPERATION_COUNT_TOTAL: u64 = 889;

const PRODUCTS: [(&str, &str); 5] = [
    ("assets", "/jsm/assets/workspace/{workspaceId}/v1"),
    ("automation", ""),
    ("platform", ""),
    ("service_desk", ""),
    ("software", ""),
];

const FIELDS_RENAMED: [(&str, &str); 8] = [
    ("async", "async_value"),
    ("includeExtendedInfo", "include_extended_information"),
    ("pos", "position"),
    ("prev", "previous"),
    ("to", "to_value"),
    ("type", "kind"),
    ("x", "x_value"),
    ("y", "y_value"),
];

const SCHEMAS_RENAMED: [(&str, &str); 1] = [("fields", "MandatoryFields")];

const WIRES_CORRECTED: [(&str, &str); 2] =
    [("The column position of the gadget.", "column"), ("The row position of the gadget.", "row")];

#[derive(Debug)]
struct Generated {
    file: PathBuf,
    text: String,
}

#[derive(Clone, Copy, Debug)]
struct Sought<'text> {
    call: &'text str,
    wire: &'text str,
}

#[derive(Clone, Copy, Debug)]
struct Specified<'document> {
    item: &'document Value,
    operation: &'document Value,
    path: &'document str,
    variant: &'static str,
}

#[test]
fn a_search_that_cuts_its_window_before_it_filters_is_never_a_walk() {
    let mut window_count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let description = specified.operation.get("description").and_then(Value::as_str);

            if !description.is_some_and(|text| text.contains("and then returns only the users")) {
                continue;
            }

            let key = format!("{} {prefix}{}", specified.variant, specified.path);
            let found = generated.get(&key).expect("a generated file");

            assert!(!found.text.contains("impl crate::Paged"), "{key} would lose users on a walk");

            window_count = window_count.strict_add(1);
        }
    }

    assert_eq!(window_count, 7, "the wording Atlassian uses for a window cut before its filter");
}

fn document_read(module: &str) -> Value {
    assert!(!module.is_empty(), "a specification is named for its module");

    let path = root().join("spec").join(format!("{module}.json"));
    let text = fs::read_to_string(&path).expect("read a pinned specification");
    let document: Value = serde_json::from_str(&text).expect("parse a pinned specification");

    assert!(document.is_object(), "a specification is an object");

    document
}

#[test]
fn every_generated_operation_comes_from_the_specification() {
    let mut count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for key in generated.keys() {
            let (variant, path) = key.split_once(' ').expect("a key is a method and a path");
            let route = path.strip_prefix(prefix).expect("a path starts with its server path");
            let item = document.get("paths").and_then(|paths| paths.get(route));
            let operation = item.and_then(|found| found.get(wire_method(variant)));

            assert!(operation.is_some(), "{key} of {module} is in no specification");

            count = count.strict_add(1);
        }
    }

    assert_eq!(count, OPERATION_COUNT_TOTAL, "every generated file was checked");
}

#[test]
fn every_member_of_every_schema_has_a_constant_on_that_schema() {
    let mut member_count = 0_u64;

    for (module, _prefix) in PRODUCTS {
        let document = document_read(module);
        let shapes = shapes_read(module);
        let schemas = document.pointer("/components/schemas").and_then(Value::as_object);

        for (name, schema) in schemas.into_iter().flatten() {
            let Some(members) = schema.get("properties").and_then(Value::as_object) else {
                continue;
            };

            let renamed = SCHEMAS_RENAMED.iter().find(|(declared, _emitted)| *declared == name);
            let emitted = renamed.map_or(name.as_str(), |(_declared, found)| *found);
            let shape = shapes.get(&name_folded(emitted));
            let text = shape.unwrap_or_else(|| panic!("{module} has no shape for {name}"));

            for wire in members.keys() {
                let corrected = WIRES_CORRECTED.iter().find(|(declared, _sent)| declared == wire);
                let named = corrected.map_or(wire.as_str(), |(_declared, sent)| *sent);
                let literal = format!("crate::Field::new(\"{named}\")");

                assert!(text.contains(&literal), "{module} {name} has no constant for {named}");

                member_count = member_count.strict_add(1);
            }
        }
    }

    assert_eq!(member_count, 5_162, "the check reached every member Atlassian documents");
}

#[test]
fn every_operation_of_every_specification_has_one_generated_file() {
    let mut count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);

            assert!(generated.contains_key(&key), "{module} has no file for {key}");

            count = count.strict_add(1);
        }
    }

    assert_eq!(count, OPERATION_COUNT_TOTAL, "the specifications hold what the crate claims");
}

#[test]
fn every_operation_that_names_the_xsrf_header_sends_it() {
    let mut named_count = 0_u64;
    let mut sent_count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);
            let found = generated.get(&key).expect("a generated file");
            let body = specified.operation.get("requestBody").map(Value::to_string);
            let is_multipart = body.unwrap_or_default().contains(MULTIPART);
            let is_named = specified.operation.to_string().contains(HEADER_XSRF_NAME);
            let is_owed = is_named && !is_multipart;
            let is_sent = found.text.contains(HEADER_XSRF_SENT);

            assert_eq!(is_sent, is_owed, "{key} sends the XSRF header {is_sent}, named {is_named}");

            named_count = named_count.strict_add(u64::from(is_named));
            sent_count = sent_count.strict_add(u64::from(is_sent));
        }
    }

    assert_eq!(named_count, 5, "three avatar uploads and two attachment uploads name the header");
    assert_eq!(sent_count, 3, "each avatar upload sends it, and each attachment body carries it");
}

#[test]
fn every_parameter_and_body_the_specification_names_reaches_the_request() {
    let mut parameter_count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);
            let found = generated.get(&key).expect("a generated file");

            parameter_count = parameter_count.strict_add(parameters_check(specified, found));

            if specified.operation.get("requestBody").is_some() {
                assert!(found.text.contains("request.body_"), "{key} drops the body");
            }
        }
    }

    assert_eq!(parameter_count, 1_753, "the check reached every parameter of every product");
}

#[test]
fn every_placeholder_of_every_path_is_filled_by_its_operation() {
    let mut placeholder_count = 0_u64;

    for (module, _prefix) in PRODUCTS {
        for found in generated_read(module).values() {
            let path = path_of(&found.text);

            for piece in path.split('{').skip(1) {
                let wire = piece.split('}').next().expect("a placeholder closes");
                let quoted = format!("(\"{wire}\", self.");

                assert!(found.text.contains(&quoted), "{} leaves {wire}", found.file.display());

                placeholder_count = placeholder_count.strict_add(1);
            }
        }
    }

    assert_eq!(placeholder_count, 780, "the check reached every placeholder of every product");
}

fn generated_read(module: &str) -> BTreeMap<String, Generated> {
    assert!(!module.is_empty(), "a module is named");

    let directory = root().join("src").join(module);
    let mut generated: BTreeMap<String, Generated> = BTreeMap::new();

    for entry in fs::read_dir(&directory).expect("read a generated module") {
        let file = entry.expect("a directory entry").path();

        if !file.is_file() {
            continue;
        }

        if file.ends_with("mod.rs") {
            continue;
        }

        let text = fs::read_to_string(&file).expect("read a generated file");
        let key = format!("{} {}", method_of(&text), path_of(&text));
        let replaced = generated.insert(key.clone(), Generated { file, text });

        assert!(replaced.is_none(), "two files of {module} claim {key}");
    }

    assert!(!generated.is_empty(), "{module} holds operations");

    generated
}

fn method_of(text: &str) -> &str {
    let tail = text.split("const METHOD: crate::Method = crate::Method::").nth(1);
    let variant = tail.and_then(|found| found.split(';').next()).expect("a method");

    assert!(!variant.is_empty(), "an operation has a method");
    assert!(variant.len() <= 6, "a method is a word");

    variant
}

fn name_folded(name: &str) -> String {
    assert!(!name.is_empty(), "a schema is named");

    let kept = name.chars().filter(char::is_ascii_alphanumeric);

    kept.map(|letter| letter.to_ascii_lowercase()).collect()
}

fn parameter_bound<'text>(
    flat: &'text str,
    sought: Sought<'_>,
) -> Option<(&'text str, &'text str)> {
    assert!(!sought.call.is_empty(), "a parameter reaches a call");
    assert!(!sought.wire.is_empty(), "a parameter is named");

    let Sought { call, wire } = sought;

    let quoted = format!("(\"{wire}\",");

    for statement in flat.split("request.").skip(1) {
        if !statement.starts_with(call) {
            continue;
        }

        let Some((_method, argument)) = statement.split_once(quoted.as_str()) else {
            continue;
        };

        let unwrapped = argument.strip_prefix("Some(").unwrap_or(argument);
        let (receiver, rest) = unwrapped.split_once('.')?;
        let end = rest.find(|found: char| !found.is_ascii_alphanumeric() && found != '_')?;

        return Some((receiver, rest.get(..end)?));
    }

    None
}

fn parameters_check(specified: Specified<'_>, found: &Generated) -> u64 {
    assert!(specified.item.is_object(), "a path item is an object");
    assert!(specified.operation.is_object(), "an operation is an object");

    let shared = specified.item.get("parameters").and_then(Value::as_array);
    let own = specified.operation.get("parameters").and_then(Value::as_array);
    let flat: String = found.text.split_whitespace().collect();
    let shown = found.file.display();
    let mut count = 0_u64;

    for parameter in shared.into_iter().flatten().chain(own.into_iter().flatten()) {
        let wire = parameter.get("name").and_then(Value::as_str).expect("a named parameter");
        let place = parameter.get("in").and_then(Value::as_str).expect("a parameter's place");

        if wire == "Authorization" {
            continue;
        }

        count = count.strict_add(1);

        if wire == TERMS_DECLARED {
            assert!(flat.contains(TERMS_SENT), "{shown} never sends its free terms");

            continue;
        }

        let call = match place {
            "header" => "header(",
            "path" => "path_",
            _ => "query_",
        };

        let bound = parameter_bound(&flat, Sought { call, wire });
        let (receiver, field) = bound.unwrap_or_else(|| panic!("{shown} never sends {wire}"));
        let renamed = FIELDS_RENAMED.iter().find(|(declared, _field)| *declared == wire);
        let expected = renamed.map_or(wire, |(_declared, emitted)| *emitted);
        let is_required = place == "path" || parameter.get("required") == Some(&Value::Bool(true));

        assert_eq!(words_of(field), words_of(expected), "{shown} binds {wire} to {field}");
        assert_eq!(receiver == "self", is_required, "{shown} misplaces {wire}");
    }

    count
}

fn path_of(text: &str) -> String {
    let tail = text.split("const PATH: &'static str =").nth(1).expect("a path");
    let statement = tail.split(';').next().expect("a path statement");
    let pieces: Vec<&str> = statement.split('"').skip(1).step_by(2).collect();
    let path = pieces.concat();

    assert!(path.starts_with('/'), "a path is rooted at its base");
    assert!(!pieces.is_empty(), "a path holds at least one literal");

    path
}

fn root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();

    assert!(root.join("Cargo.toml").is_file(), "the manifest directory is the repository");
    assert!(root.join("spec").is_dir(), "the repository pins its specifications");

    root
}

fn shapes_read(module: &str) -> BTreeMap<String, String> {
    assert!(!module.is_empty(), "a module is named");

    let directory = root().join("src").join(module).join("shape");
    let mut shapes: BTreeMap<String, String> = BTreeMap::new();

    for entry in fs::read_dir(&directory).expect("read a shape module") {
        let file = entry.expect("a directory entry").path();

        if file.ends_with("mod.rs") {
            continue;
        }

        let text = fs::read_to_string(&file).expect("read a shape");
        let tail = text.split("pub struct ").nth(1).expect("a shape is a struct");
        let name = tail.split([';', ' ', '<']).next().expect("a struct is named");
        let replaced = shapes.insert(name_folded(name), text.clone());

        assert!(replaced.is_none(), "two shapes of {module} fold to {name}");
    }

    assert!(!shapes.is_empty(), "{module} holds shapes");

    shapes
}

fn specified_read(document: &Value) -> Vec<Specified<'_>> {
    assert!(document.is_object(), "a specification is an object");

    let paths = document.get("paths").and_then(Value::as_object).expect("paths");
    let mut specified: Vec<Specified<'_>> = Vec::new();

    for (path, item) in paths {
        for (method, variant) in METHODS {
            if let Some(operation) = item.get(method) {
                specified.push(Specified { item, operation, path, variant });
            }
        }
    }

    assert!(!specified.is_empty(), "a specification names operations");

    specified
}

fn wire_method(variant: &str) -> &'static str {
    assert!(!variant.is_empty(), "a method is named");

    let found = METHODS.iter().find(|(_wire, name)| *name == variant).expect("a known method");

    assert_eq!(found.1, variant, "the table names the variant");

    found.0
}

fn word_starts_at(letters: &[char], index: usize) -> bool {
    assert!(index < letters.len(), "a letter sits inside its name");

    if !letters.get(index).is_some_and(char::is_ascii_uppercase) {
        return false;
    }

    let before = index.checked_sub(1).and_then(|at| letters.get(at));

    if before.is_some_and(|found| !found.is_ascii_uppercase()) {
        return true;
    }

    letters.get(index.strict_add(1)).is_some_and(char::is_ascii_lowercase)
}

fn words_of(name: &str) -> Vec<String> {
    assert!(!name.is_empty(), "a name holds a word");

    let letters: Vec<char> = name.chars().collect();
    let mut words: Vec<String> = vec![String::new()];

    for (index, letter) in letters.iter().enumerate() {
        if !letter.is_ascii_alphanumeric() {
            words.push(String::new());

            continue;
        }

        if word_starts_at(&letters, index) {
            words.push(String::new());
        }

        words.last_mut().expect("a word").push(letter.to_ascii_lowercase());
    }

    words.retain(|word| !word.is_empty());
    words.sort();

    assert!(!words.is_empty(), "{name} holds a word");

    words
}
