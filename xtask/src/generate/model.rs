use alloc::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use super::name::{constant_name, field_name, type_name};
use super::overrides::{
    ANY_REPLIES,
    BARE_WALKS,
    BINARY_REPLIES,
    BODIES_UNDECLARED,
    FIELD_KINDS,
    FIELD_WIRES,
    FIELDS_UNDECLARED,
    FREE_TERMS,
    INLINE_NAMES,
    JSON_BODIES,
    PAGING_ITEMS,
    READS_BY_POST,
    SCHEMA_NAMES,
    TYPE_NAMES,
    WINDOW_FIRST,
};
use super::product::Product;

const ALL_OF_MEMBER_COUNT_MAX: u32 = 64;

const BODY_KINDS: [(&str, BodyKind); 4] = [
    (CONTENT_JSON, BodyKind::JSONValue),
    ("application/json-patch+json", BodyKind::JSONPatch),
    ("multipart/form-data", BodyKind::Multipart),
    ("text/plain", BodyKind::Text),
];

const CONTENT_JSON: &str = "application/json";

const CURSOR_TOKEN: &str = "cursor";

const CURSORS: [(&str, CursorKind); 8] = [
    ("after", CursorKind::Link),
    (CURSOR_TOKEN, CursorKind::Token),
    ("nextPageToken", CursorKind::Token),
    ("offset", CursorKind::Offset),
    ("page", CursorKind::Number),
    ("since", CursorKind::Link),
    ("start", CursorKind::Offset),
    ("startAt", CursorKind::Offset),
];

const FREE_TERMS_DECLARED: &str = "params";
const FREE_TERMS_FIELD: &str = "properties";
const HEADER_EXPERIMENTAL: (&str, &str) = ("X-ExperimentalApi", "opt-in");
const HEADER_XSRF: (&str, &str) = ("X-Atlassian-Token", "no-check");

const ITEM_FIELDS: [&str; 12] = [
    "values",
    "issues",
    "comments",
    "worklogs",
    "dashboards",
    "entries",
    "records",
    "data",
    "fields",
    "results",
    "issueTypes",
    "objectEntries",
];

const KIND_ANY: &str = "crate::Any";
const KIND_BINARY: &str = "crate::Binary";
const KIND_DEPTH_MAX: u32 = 32;
const KIND_NOTHING: &str = "crate::Nothing";
const LINK_HOLDERS: [&str; 2] = ["_links", "links"];
const LINKS_OUTER: [&str; 2] = ["next", "nextPage"];

const METHODS: [(&str, &str); 5] =
    [("delete", "Delete"), ("get", "Get"), ("patch", "Patch"), ("post", "Post"), ("put", "Put")];

const PENDING_ROUND_COUNT_MAX: u32 = 1_000_000;
const SCHEMA_PREFIX: &str = "#/components/schemas/";
pub(crate) const SCHEMA_SCOPE: &str = "SCHEMA::";
const STATUS_NO_CONTENT: &str = "204";
const TOKEN_CONTAINER_ITEMS: &str = "values";
const TOKEN_FIELDS: [&str; 2] = ["nextPageCursor", "nextPageToken"];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Body {
    pub is_required: bool,
    pub kind: BodyKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BodyKind {
    JSONPatch,
    JSONValue,
    Multipart,
    Text,
    Upload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CursorKind {
    Link,
    Number,
    Offset,
    Token,
}

#[derive(Debug)]
struct ItemField<'document> {
    items: String,
    schema: Option<&'document Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Location {
    Header,
    Path,
    Query,
}

#[derive(Debug)]
pub(crate) struct Model {
    pub identifiers: BTreeSet<String>,
    pub operations: Vec<Operation>,
    pub schemas: BTreeMap<String, Vec<SchemaField>>,
}

#[derive(Debug)]
pub(crate) struct Operation {
    pub body: Option<Body>,
    pub headers: Vec<(&'static str, &'static str)>,
    pub is_read: bool,
    pub method: &'static str,
    pub name: String,
    pub paging: Option<Paging>,
    pub parameters: Vec<Parameter>,
    pub path: String,
    pub reply: String,
}

#[derive(Debug)]
pub(crate) struct Paging {
    pub cursor: String,
    pub item: String,
    pub items: Option<String>,
    pub next: String,
}

#[derive(Debug)]
pub(crate) struct Parameter {
    pub is_required: bool,
    pub kind: ParameterKind,
    pub location: Location,
    pub name: String,
    pub wire: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParameterKind {
    Boolean,
    Number,
    Numbers,
    Terms,
    Text,
    Texts,
}

#[derive(Debug)]
struct PageSource<'document> {
    field: ItemField<'document>,
    next: String,
}

#[derive(Debug)]
struct Pending<'document> {
    name: String,
    node: &'document Value,
}

#[derive(Debug)]
struct Registry<'document> {
    document: &'document Value,
    names: BTreeMap<String, &'document Value>,
    pending: Vec<Pending<'document>>,
    schemas: BTreeMap<String, Vec<SchemaField>>,
}

#[derive(Debug)]
pub(crate) struct SchemaField {
    pub constant: String,
    pub kind: String,
    pub wire: String,
}

#[derive(Clone, Copy, Debug)]
struct Spot<'document> {
    item: &'document Value,
    method: &'static str,
    operation: &'document Value,
    path: &'document str,
    variant: &'static str,
}

#[derive(Debug)]
enum Step<'document> {
    Follow(&'document Value),
    Leaf(String),
    Wrap(&'static str, &'document Value),
}

impl<'document> Registry<'document> {
    fn drain(&mut self) {
        assert!(self.document.is_object(), "a registry reads a document");
        assert!(self.names.len() >= self.schemas.len(), "every schema was registered first");

        for _round_index in 0..PENDING_ROUND_COUNT_MAX {
            let Some(pending) = self.pending.pop() else {
                break;
            };

            let fields = self.fields_read(&pending);
            let replaced = self.schemas.insert(pending.name, fields);

            assert!(replaced.is_none(), "a schema is emitted once");
        }

        assert!(self.pending.is_empty(), "every schema was read within the budget");
    }

    fn fields_read(&mut self, pending: &Pending<'document>) -> Vec<SchemaField> {
        assert!(!pending.name.is_empty(), "a schema is named");

        let properties = self.properties_read(pending.node);
        let mut by_constant: BTreeMap<String, SchemaField> = BTreeMap::new();

        for (declared, node) in properties {
            let declared_kind = self.kind(node, &inline_name(pending, declared));
            let kind = field_kind(pending, declared, declared_kind);
            let wire = field_wire(pending, declared);
            let constant = constant_name(wire);
            let field = SchemaField { constant: constant.clone(), kind, wire: wire.to_owned() };
            let replaced = by_constant.insert(constant, field);

            assert!(replaced.is_none(), "two members of {} share a name", pending.name);
        }

        for (_schema, wire, kind) in fields_undeclared(pending) {
            let field = SchemaField {
                constant: constant_name(wire),
                kind: (*kind).to_owned(),
                wire: (*wire).to_owned(),
            };

            let _held = by_constant.entry(field.constant.clone()).or_insert(field);
        }

        by_constant.into_values().collect()
    }

    fn kind(&mut self, node: &'document Value, inline_name: &str) -> String {
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        let mut wrappers: Vec<&'static str> = Vec::new();
        let mut current = node;
        let mut text = KIND_ANY.to_owned();

        assert!(current.is_object() || current.is_boolean(), "a schema is an object or a boolean");

        for _depth_index in 0..KIND_DEPTH_MAX {
            let step = self.step(current, inline_name);

            if let Step::Leaf(found) = step {
                text = found;

                break;
            }

            if let Step::Wrap(wrapper, _inner) = step {
                wrappers.push(wrapper);
            }

            current = step_inner(&step, current);
        }

        for wrapper in wrappers.iter().rev() {
            text = format!("crate::{wrapper}<{text}>");
        }

        assert!(text.starts_with("crate::"), "a kind is a runtime type");
        assert!(wrappers.len() <= 32, "a kind peels within its budget");

        text
    }

    fn new(document: &'document Value) -> Self {
        assert!(document.is_object(), "a specification is an object");

        let mut registry = Self {
            document,
            names: BTreeMap::new(),
            pending: Vec::new(),
            schemas: BTreeMap::new(),
        };

        let schemas = document.pointer("/components/schemas").and_then(Value::as_object);

        for (key, schema) in schemas.into_iter().flatten() {
            if record_is(schema) {
                registry.record_register(schema_name(key), schema);
            }
        }

        assert_eq!(registry.names.len(), registry.pending.len(), "each schema is registered once");

        registry
    }

    fn properties_read(&self, node: &'document Value) -> Vec<(&'document str, &'document Value)> {
        assert!(self.document.is_object(), "a registry reads a document");

        let mut found: BTreeMap<&'document str, &'document Value> = BTreeMap::new();
        let mut stack: Vec<&'document Value> = vec![node];

        for _member_index in 0..ALL_OF_MEMBER_COUNT_MAX {
            let Some(current) = stack.pop() else {
                break;
            };

            let resolved = self.resolved(current);
            let properties = resolved.get("properties").and_then(Value::as_object);

            for (wire, property) in properties.into_iter().flatten() {
                let _replaced = found.insert(wire.as_str(), property);
            }

            let members = resolved.get("allOf").and_then(Value::as_array);

            stack.extend(members.into_iter().flatten());
        }

        assert!(stack.is_empty(), "every allOf member was read within the budget");

        found.into_iter().collect()
    }

    fn record_register(&mut self, name: String, node: &'document Value) {
        assert!(!name.is_empty(), "a schema is named");
        assert!(record_is(node), "only a record is registered");

        if let Some(known) = self.names.get(&name) {
            assert_eq!(*known, node, "two different schemas are both named {name}");

            return;
        }

        let _absent = self.names.insert(name.clone(), node);

        self.pending.push(Pending { name, node });
    }

    fn resolved(&self, node: &'document Value) -> &'document Value {
        assert!(self.document.is_object(), "a registry reads a document");

        let Some(reference) = node.get("$ref").and_then(Value::as_str) else {
            return node;
        };

        assert!(reference.starts_with(SCHEMA_PREFIX), "a reference names a schema");

        self.document.pointer(reference.trim_start_matches('#')).unwrap_or(node)
    }

    fn step(&mut self, node: &'document Value, inline_name: &str) -> Step<'document> {
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
            return self.step_of_reference(reference, self.resolved(node));
        }

        if let Some(only) = all_of_single(node) {
            return Step::Follow(only);
        }

        if record_is(node) {
            self.record_register(inline_name.to_owned(), node);

            return Step::Leaf(format!("crate::Object<{SCHEMA_SCOPE}{inline_name}>"));
        }

        step_of_type(node)
    }

    fn step_of_reference(
        &mut self,
        reference: &str,
        resolved: &'document Value,
    ) -> Step<'document> {
        assert!(reference.starts_with(SCHEMA_PREFIX), "a reference names a schema");

        if !record_is(resolved) {
            return Step::Follow(resolved);
        }

        let key = reference.trim_start_matches(SCHEMA_PREFIX);
        let name = schema_name(key);

        if key.contains('/') {
            self.record_register(name.clone(), resolved);
        }

        Step::Leaf(format!("crate::Object<{SCHEMA_SCOPE}{name}>"))
    }
}

impl<'document> Spot<'document> {
    fn identifier(&self) -> &'document str {
        assert!(!self.path.is_empty(), "an operation has a path");
        assert!(self.operation.is_object(), "an operation is an object");

        self.operation.get("operationId").and_then(Value::as_str).unwrap_or_default()
    }

    fn is_read(&self) -> bool {
        assert!(!self.method.is_empty(), "an operation has a method");
        assert!(!self.path.is_empty(), "an operation has a path");

        if self.method == "get" {
            return true;
        }

        READS_BY_POST.contains(&self.identifier())
    }

    fn key(&self) -> String {
        assert!(!self.method.is_empty(), "an operation has a method");
        assert!(!self.path.is_empty(), "an operation has a path");

        format!("{} {}", self.method, self.path)
    }
}

fn all_of_single(node: &Value) -> Option<&Value> {
    let members = node.get("allOf").and_then(Value::as_array)?;

    let [only] = members.as_slice() else {
        return None;
    };

    assert!(only.is_object(), "an allOf member is a schema");
    assert!(!members.is_empty(), "a single member comes from a filled list");

    Some(only)
}

fn body_of(spot: Spot<'_>) -> Option<Body> {
    assert!(spot.operation.is_object(), "an operation is an object");

    if BODIES_UNDECLARED.contains(&spot.key().as_str()) {
        return Some(Body { is_required: true, kind: BodyKind::JSONValue });
    }

    body_read(spot.operation)
}

fn body_read(operation: &Value) -> Option<Body> {
    assert!(operation.is_object(), "an operation is an object");

    let request_body = operation.get("requestBody")?;

    assert!(request_body.get("$ref").is_none(), "a referenced request body needs resolving");

    let content = request_body.get("content").and_then(Value::as_object)?;
    let is_required = request_body.get("required").and_then(Value::as_bool).unwrap_or_default();
    let found = BODY_KINDS.iter().find(|(name, _kind)| content.contains_key(*name));
    let identifier = operation.get("operationId").and_then(Value::as_str).unwrap_or_default();

    let kind = if JSON_BODIES.contains(&identifier) {
        BodyKind::JSONValue
    } else {
        found.map_or(BodyKind::Upload, |(_name, kind)| *kind)
    };

    assert!(!content.is_empty(), "a request body names its content");

    Some(Body { is_required, kind })
}

fn cursor_find(names: &[&str]) -> Option<(String, CursorKind)> {
    assert!(!CURSORS.is_empty(), "a walk has parameters to look for");

    let found = CURSORS.iter().rev().find(|(name, _kind)| names.contains(name))?;

    assert!(!found.0.is_empty(), "a cursor is named");
    assert!(names.contains(&found.0), "a cursor is one of the operation's parameters");

    Some((found.0.to_owned(), found.1))
}

fn cursor_refined<'document>(
    registry: &Registry<'document>,
    names: &[&str],
    properties: &[(&'document str, &'document Value)],
    found: (String, CursorKind),
) -> (String, CursorKind) {
    assert!(!names.is_empty(), "a walk has parameters");
    assert!(!found.0.is_empty(), "a cursor is named");

    if !names.contains(&CURSOR_TOKEN) {
        return found;
    }

    if token_field_find(properties).is_some() {
        return (CURSOR_TOKEN.to_owned(), CursorKind::Token);
    }

    if page_source_link(registry, properties).is_some() {
        return (CURSOR_TOKEN.to_owned(), CursorKind::Link);
    }

    found
}

fn field_kind(pending: &Pending<'_>, wire: &str, declared: String) -> String {
    let schema = pending.name.as_str();

    assert!(!schema.is_empty(), "a field belongs to a schema");
    assert!(!declared.is_empty(), "a field has a kind");

    let fixed = FIELD_KINDS.iter().find(|(name, member, _kind)| *name == schema && *member == wire);

    fixed.map_or(declared, |(_name, _member, kind)| (*kind).to_owned())
}

fn field_wire<'wire>(pending: &Pending<'_>, declared: &'wire str) -> &'wire str {
    let schema = pending.name.as_str();

    assert!(!schema.is_empty(), "a field belongs to a schema");
    assert!(!declared.is_empty(), "a field is named");

    let fixed =
        FIELD_WIRES.iter().find(|(name, member, _wire)| *name == schema && *member == declared);

    fixed.map_or(declared, |(_name, _member, wire)| *wire)
}

fn fields_undeclared<'table>(
    pending: &Pending<'_>,
) -> impl Iterator<Item = &'table (&'static str, &'static str, &'static str)> {
    let schema = pending.name.clone();

    assert!(!schema.is_empty(), "a field belongs to a schema");

    FIELDS_UNDECLARED.iter().filter(move |(name, _wire, _kind)| *name == schema)
}

fn header_experimental(spot: Spot<'_>, product: &Product) -> Option<(&'static str, &'static str)> {
    assert!(spot.operation.is_object(), "an operation is an object");

    if !product.is_experimental_header_sent {
        return None;
    }

    let is_experimental = spot.operation.get("x-experimental").and_then(Value::as_bool);

    is_experimental.unwrap_or_default().then_some(HEADER_EXPERIMENTAL)
}

fn header_xsrf(spot: Spot<'_>) -> Option<(&'static str, &'static str)> {
    assert!(spot.operation.is_object(), "an operation is an object");

    if body_of(spot).is_some_and(|body| body.kind == BodyKind::Multipart) {
        return None;
    }

    let description = spot.operation.get("description").and_then(Value::as_str);

    description.unwrap_or_default().contains(HEADER_XSRF.0).then_some(HEADER_XSRF)
}

fn headers_read(spot: Spot<'_>, product: &Product) -> Vec<(&'static str, &'static str)> {
    assert!(spot.operation.is_object(), "an operation is an object");

    let named = header_xsrf(spot).into_iter().chain(header_experimental(spot, product));
    let headers: Vec<(&'static str, &'static str)> = named.collect();

    assert!(headers.len() <= 2, "an operation adds at most the two headers Jira names");

    headers
}

fn identifiers_read(paths: &Map<String, Value>) -> BTreeSet<String> {
    assert!(!paths.is_empty(), "a specification names paths");

    let mut identifiers: BTreeSet<String> = BTreeSet::new();

    for (method, _variant) in METHODS {
        let operations = paths.values().filter_map(|item| item.get(method));
        let held = paths.iter().filter(|(_path, item)| item.get(method).is_some());

        identifiers.extend(
            operations
                .filter_map(|operation| operation.get("operationId"))
                .filter_map(Value::as_str)
                .map(str::to_owned),
        );

        identifiers.extend(held.map(|(path, _item)| format!("{method} {path}")));
    }

    assert!(!identifiers.is_empty(), "a specification names operations");

    identifiers
}

fn inline_name(pending: &Pending<'_>, wire: &str) -> String {
    assert!(!pending.name.is_empty(), "an inline record sits inside a named record");
    assert!(!wire.is_empty(), "an inline record sits under a named member");

    let name = format!("{}{}", pending.name, type_name(wire));
    let found = INLINE_NAMES.iter().find(|(full, _short)| *full == name);

    found.map_or(name, |(_full, short)| (*short).to_owned())
}

fn item_field_find<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<ItemField<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    for field in ITEM_FIELDS {
        let found = properties.iter().find(|(wire, _node)| *wire == field);

        if let Some((wire, node)) = found {
            let resolved = registry.resolved(node);

            if resolved.get("type").and_then(Value::as_str) == Some("array") {
                return Some(ItemField {
                    items: format!("/{wire}"),
                    schema: resolved.get("items"),
                });
            }
        }
    }

    None
}

fn items_overridden(identifier: &str) -> Option<String> {
    assert!(!PAGING_ITEMS.is_empty(), "the overrides name at least one operation");

    let found = PAGING_ITEMS.iter().find(|(id, _items)| *id == identifier);
    let items = found.map(|(_id, items)| (*items).to_owned());

    assert!(items.as_ref().is_none_or(|name| !name.is_empty()), "an array has a name");

    items
}

const fn location_rank(location: Location) -> u32 {
    match location {
        Location::Header => 0,
        Location::Path => 1,
        Location::Query => 2,
    }
}

pub(crate) fn model_build(product: &Product, document: &Value) -> Result<Model, String> {
    assert!(document.is_object(), "a specification is an object");
    assert!(!product.module.is_empty(), "a product names its module");

    let mut registry = Registry::new(document);
    let mut operations: Vec<Operation> = Vec::new();

    let paths = document
        .get("paths")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{} names no paths", product.module))?;

    for (path, item) in paths {
        operations.extend(operations_read(&mut registry, product, (path, item))?);
    }

    registry.drain();

    operations.sort_by(|left, right| left.name.cmp(&right.name));

    let names: BTreeSet<&String> = operations.iter().map(|found| &found.name).collect();

    if names.len() != operations.len() {
        return Err(format!("two operations of {} share a name", product.module));
    }

    assert!(!operations.is_empty(), "a product holds operations");

    Ok(Model { identifiers: identifiers_read(paths), operations, schemas: registry.schemas })
}

fn name_read(spot: Spot<'_>) -> Result<String, String> {
    assert!(!spot.path.is_empty(), "an operation has a path");

    let overridden =
        TYPE_NAMES.iter().find(|(verb, route, _name)| (*verb, *route) == (spot.method, spot.path));

    if let Some((_verb, _route, name)) = overridden {
        return Ok((*name).to_owned());
    }

    let identifier = spot.identifier();

    if identifier.is_empty() {
        return Err(format!("{} {} names no operationId", spot.method, spot.path));
    }

    Ok(type_name(identifier))
}

fn operation_read<'document>(
    registry: &mut Registry<'document>,
    product: &Product,
    spot: Spot<'document>,
) -> Result<Operation, String> {
    assert!(!spot.path.is_empty(), "an operation has a path");
    assert!(spot.operation.is_object(), "an operation is an object");

    let name = name_read(spot)?;
    let mut parameters = parameters_read(registry, spot);

    let path = format!("{}{}", product.path_prefix, spot.path);

    placeholders_declare(&path, &mut parameters);
    placeholders_check(&path, &parameters)?;

    let is_binary = reply_is_binary(spot);
    let reply_node = if is_binary { None } else { reply_schema(spot.operation) };

    let reply = if is_binary {
        KIND_BINARY.to_owned()
    } else {
        reply_kind(registry, spot, reply_node, &name)
    };

    let paging = paging_read(registry, spot, &parameters, reply_node, &name);

    parameters.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(Operation {
        body: body_of(spot),
        headers: headers_read(spot, product),
        is_read: spot.is_read(),
        method: spot.variant,
        name,
        paging,
        parameters,
        path,
        reply,
    })
}

fn operations_read<'document>(
    registry: &mut Registry<'document>,
    product: &Product,
    (path, item): (&'document str, &'document Value),
) -> Result<Vec<Operation>, String> {
    assert!(!path.is_empty(), "a path is named");
    assert!(item.is_object(), "a path item is an object");

    let mut operations: Vec<Operation> = Vec::new();

    for (method, variant) in METHODS {
        if let Some(operation) = item.get(method) {
            let spot = Spot { item, method, operation, path, variant };

            operations.push(operation_read(registry, product, spot)?);
        }
    }

    assert!(operations.len() <= METHODS.len(), "a path holds one operation per method");

    Ok(operations)
}

fn page_source_link<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<PageSource<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    let links = properties.iter().find(|(wire, _node)| LINK_HOLDERS.contains(wire));
    let inner = links.map(|(_wire, node)| registry.properties_read(node)).unwrap_or_default();
    let has_inner = inner.iter().any(|(wire, _node)| *wire == "next");
    let outer = properties.iter().find(|(wire, _node)| LINKS_OUTER.contains(wire));

    let pointer = match (links.filter(|_found| has_inner), outer) {
        (Some((wire, _node)), _) => format!("/{wire}/next"),
        (None, Some((wire, _node))) => format!("/{wire}"),
        (None, None) => return None,
    };

    let field = item_field_find(registry, properties)?;

    assert!(pointer.starts_with('/'), "a link is named by a pointer");

    Some(PageSource { field, next: format!("crate::Next::Link(\"{pointer}\")") })
}

fn page_source_nested<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<PageSource<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    for (wire, node) in properties {
        let inner = registry.properties_read(node);

        if let Some(token) = token_field_find(&inner) {
            let items = inner.iter().find(|(name, _inner)| *name == TOKEN_CONTAINER_ITEMS)?;
            let resolved = registry.resolved(items.1);

            return Some(PageSource {
                field: ItemField {
                    items: format!("/{wire}/{TOKEN_CONTAINER_ITEMS}"),
                    schema: resolved.get("items"),
                },
                next: format!("crate::Next::Token(\"/{wire}/{token}\")"),
            });
        }
    }

    None
}

fn page_source_read<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
    kind: CursorKind,
) -> Option<PageSource<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    if kind == CursorKind::Link {
        return page_source_link(registry, properties);
    }

    if kind == CursorKind::Number {
        let field = item_field_find(registry, properties)?;

        return Some(PageSource { field, next: "crate::Next::Number".to_owned() });
    }

    if kind == CursorKind::Offset {
        let field = item_field_find(registry, properties)?;

        return Some(PageSource { field, next: "crate::Next::Offset".to_owned() });
    }

    if let Some(token) = token_field_find(properties) {
        let field = item_field_find(registry, properties)?;

        return Some(PageSource { field, next: format!("crate::Next::Token(\"/{token}\")") });
    }

    page_source_nested(registry, properties).or_else(|| page_source_link(registry, properties))
}

fn paging_of_list<'document>(
    registry: &mut Registry<'document>,
    (cursor, kind): (String, CursorKind),
    (identifier, reply): (&str, &'document Value),
    item_name: &str,
) -> Option<Paging> {
    assert!(!cursor.is_empty(), "a walk names its cursor");
    assert!(!item_name.is_empty(), "an inline item is named");

    if kind != CursorKind::Offset {
        return None;
    }

    if !BARE_WALKS.contains(&identifier) {
        return None;
    }

    let item = reply
        .get("items")
        .map_or_else(|| KIND_ANY.to_owned(), |items| registry.kind(items, item_name));

    Some(Paging { cursor, item, items: None, next: "crate::Next::Offset".to_owned() })
}

fn paging_read<'document>(
    registry: &mut Registry<'document>,
    spot: Spot<'document>,
    parameters: &[Parameter],
    reply: Option<&'document Value>,
    name: &str,
) -> Option<Paging> {
    assert!(!name.is_empty(), "an operation is named");

    if spot.method != "get" {
        return None;
    }

    if WINDOW_FIRST.contains(&spot.identifier()) {
        return None;
    }

    let names: Vec<&str> = parameters.iter().map(|found| found.wire.as_str()).collect();
    let (cursor, kind) = cursor_find(&names)?;
    let item_name = format!("{name}Item");

    let Some(node) = reply else {
        let items = Some(items_overridden(spot.identifier())?);
        let next = "crate::Next::Offset".to_owned();

        return Some(Paging { cursor, item: KIND_ANY.to_owned(), items, next });
    };

    let resolved = registry.resolved(node);

    if resolved.get("type").and_then(Value::as_str) == Some("array") {
        let list = (spot.identifier(), resolved);

        return paging_of_list(registry, (cursor, kind), list, &item_name);
    }

    let properties = registry.properties_read(resolved);
    let refined = cursor_refined(registry, &names, &properties, (cursor, kind));
    let source = page_source_read(registry, &properties, refined.1)?;
    let schema = source.field.schema;
    let item = schema.map_or_else(|| KIND_ANY.to_owned(), |found| registry.kind(found, &item_name));

    Some(Paging { cursor: refined.0, item, items: Some(source.field.items), next: source.next })
}

fn parameter_kind(registry: &Registry<'_>, schema: &Value) -> ParameterKind {
    assert!(registry.document.is_object(), "a registry reads a document");

    let resolved = registry.resolved(schema);

    match resolved.get("type").and_then(Value::as_str) {
        Some("array") => {
            let items = resolved.get("items").map(|found| registry.resolved(found));
            let item_type = items.and_then(|found| found.get("type")).and_then(Value::as_str);

            if item_type == Some("integer") {
                return ParameterKind::Numbers;
            }

            ParameterKind::Texts
        }
        Some("boolean") => ParameterKind::Boolean,
        Some("integer") => ParameterKind::Number,
        _ => ParameterKind::Text,
    }
}

fn parameter_read(registry: &Registry<'_>, raw: &Value) -> Option<Parameter> {
    assert!(registry.document.is_object(), "a registry reads a document");

    let resolved = registry.resolved(raw);
    let wire = resolved.get("name").and_then(Value::as_str).unwrap_or_default();
    let place = resolved.get("in").and_then(Value::as_str).unwrap_or_default();

    assert!(!wire.is_empty(), "a parameter is named");

    if wire.eq_ignore_ascii_case("Authorization") {
        return None;
    }

    let location = match place {
        "header" => Location::Header,
        "path" => Location::Path,
        "query" => Location::Query,
        _ => unknown_location(wire),
    };

    let explode = resolved.get("explode").and_then(Value::as_bool);

    assert!(explode.is_none_or(|found| found), "{wire} needs a comma list the emitter lacks");

    let is_required = resolved.get("required").and_then(Value::as_bool).unwrap_or_default();
    let schema = resolved.get("schema").unwrap_or(&Value::Null);

    if location == Location::Header {
        assert!(is_required, "the optional header {wire} needs a call the emitter does not have");
    }

    Some(Parameter {
        is_required: is_required || location == Location::Path,
        kind: parameter_kind(registry, schema),
        location,
        name: field_name(wire),
        wire: wire.to_owned(),
    })
}

fn parameters_read(registry: &Registry<'_>, spot: Spot<'_>) -> Vec<Parameter> {
    assert!(spot.operation.is_object(), "an operation is an object");
    assert!(spot.item.is_object(), "a path item is an object");

    let shared = spot.item.get("parameters").and_then(Value::as_array);
    let own = spot.operation.get("parameters").and_then(Value::as_array);
    let mut by_wire: BTreeMap<(String, u32), Parameter> = BTreeMap::new();

    for raw in shared.into_iter().flatten().chain(own.into_iter().flatten()) {
        if let Some(parameter) = parameter_read(registry, raw) {
            let place = location_rank(parameter.location);
            let _replaced = by_wire.insert((parameter.wire.clone(), place), parameter);
        }
    }

    let parameters: Vec<Parameter> = by_wire.into_values().collect();

    if !FREE_TERMS.contains(&spot.identifier()) {
        return parameters;
    }

    terms_declared(parameters)
}

fn placeholders_check(path: &str, parameters: &[Parameter]) -> Result<(), String> {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    let wires = parameters.iter().filter(|found| found.location == Location::Path);

    for parameter in wires {
        if !path.contains(&format!("{{{}}}", parameter.wire)) {
            return Err(format!("{path} never names the path parameter {}", parameter.wire));
        }
    }

    assert!(!path.contains('?'), "a query term belongs in the parameters");

    Ok(())
}

fn placeholders_declare(path: &str, parameters: &mut Vec<Parameter>) {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    for piece in path.split('{').skip(1) {
        let wire = piece.split('}').next().unwrap_or_default();

        assert!(!wire.is_empty(), "a placeholder is named");

        if !parameters.iter().any(|found| found.wire == wire) {
            parameters.push(Parameter {
                is_required: true,
                kind: ParameterKind::Text,
                location: Location::Path,
                name: field_name(wire),
                wire: wire.to_owned(),
            });
        }
    }
}

fn record_is(node: &Value) -> bool {
    assert!(node.is_object() || node.is_boolean(), "a schema is an object or a boolean");

    let properties = node.get("properties").and_then(Value::as_object);
    let members = node.get("allOf").and_then(Value::as_array);

    if properties.is_some_and(|found| !found.is_empty()) {
        return true;
    }

    assert!(properties.is_none_or(Map::is_empty), "a record with members already passed");

    members.is_some_and(|found| found.len() >= 2)
}

fn reply_is_binary(spot: Spot<'_>) -> bool {
    assert!(spot.operation.is_object(), "an operation is an object");

    if BINARY_REPLIES.contains(&spot.identifier()) {
        return true;
    }

    let Some(responses) = spot.operation.get("responses").and_then(Value::as_object) else {
        return false;
    };

    let contents = responses_success(responses)
        .filter_map(|(_status, response)| response.get("content").and_then(Value::as_object));

    let named: Vec<&Map<String, Value>> = contents.filter(|found| !found.is_empty()).collect();

    if named.is_empty() {
        return false;
    }

    !named.iter().any(|content| content.contains_key(CONTENT_JSON))
}

fn reply_kind<'document>(
    registry: &mut Registry<'document>,
    spot: Spot<'document>,
    reply: Option<&'document Value>,
    name: &str,
) -> String {
    assert!(!name.is_empty(), "an operation is named");
    assert!(spot.operation.is_object(), "an operation is an object");

    if ANY_REPLIES.contains(&spot.identifier()) {
        return KIND_ANY.to_owned();
    }

    if let Some(node) = reply {
        return registry.kind(node, &format!("{name}Reply"));
    }

    let Some(responses) = spot.operation.get("responses").and_then(Value::as_object) else {
        return KIND_NOTHING.to_owned();
    };

    let Some(status) = status_success(responses) else {
        return KIND_NOTHING.to_owned();
    };

    let response = responses.get(status);
    let content = response.and_then(|found| found.get("content")).and_then(Value::as_object);

    if content.is_none_or(Map::is_empty) {
        return KIND_NOTHING.to_owned();
    }

    if status == STATUS_NO_CONTENT {
        return KIND_NOTHING.to_owned();
    }

    KIND_ANY.to_owned()
}

fn reply_schema(operation: &Value) -> Option<&Value> {
    assert!(operation.is_object(), "an operation is an object");

    let responses = operation.get("responses").and_then(Value::as_object)?;

    for (status, response) in responses_success(responses) {
        assert!(response.get("$ref").is_none(), "a referenced {status} reply needs resolving");

        let schema = response.pointer("/content/application~1json/schema");

        if let Some(found) = schema.filter(|named| schema_names_something(named)) {
            return Some(found);
        }
    }

    None
}

fn responses_success(responses: &Map<String, Value>) -> impl Iterator<Item = (&String, &Value)> {
    assert!(!responses.is_empty(), "an operation names its responses");

    responses.iter().filter(|(status, _response)| status.starts_with(['2', '3']))
}

fn schema_name(key: &str) -> String {
    assert!(!key.is_empty(), "a schema is named");

    let found = SCHEMA_NAMES.iter().find(|(wire, _name)| *wire == key);
    let name = found.map_or_else(|| type_name(key), |(_wire, name)| (*name).to_owned());

    assert!(!name.is_empty(), "a schema type is named");

    name
}

fn schema_names_something(schema: &Value) -> bool {
    assert!(schema.is_object() || schema.is_boolean(), "a schema is an object or a boolean");

    schema.as_object().is_some_and(|found| !found.is_empty())
}

fn status_success(responses: &Map<String, Value>) -> Option<&str> {
    let statuses = responses.keys().map(String::as_str);
    let found = statuses.filter(|status| status.starts_with(['2', '3'])).min();

    assert!(found.is_none_or(|status| !status.is_empty()), "a status is never empty");
    assert!(found.is_none_or(|status| responses.contains_key(status)), "a status is a response");

    found
}

fn step_inner<'document>(step: &Step<'document>, current: &'document Value) -> &'document Value {
    assert!(current.is_object() || current.is_boolean(), "a schema is an object or a boolean");

    match *step {
        Step::Follow(inner) | Step::Wrap(_, inner) => inner,
        Step::Leaf(_) => current,
    }
}

fn step_of_type(node: &Value) -> Step<'_> {
    assert!(node.is_object() || node.is_boolean(), "a schema is an object or a boolean");

    match node.get("type").and_then(Value::as_str) {
        Some("array") => node.get("items").map_or_else(
            || Step::Leaf(format!("crate::List<{KIND_ANY}>")),
            |items| Step::Wrap("List", items),
        ),
        Some("boolean") => Step::Leaf("crate::Boolean".to_owned()),
        Some("integer") => Step::Leaf("crate::Integer".to_owned()),
        Some("number") => Step::Leaf("crate::Number".to_owned()),
        Some("string") => Step::Leaf("crate::Text".to_owned()),
        _ => match node.get("additionalProperties") {
            Some(inner) if inner.is_object() => Step::Wrap("Map", inner),
            Some(Value::Bool(true)) => Step::Leaf(format!("crate::Map<{KIND_ANY}>")),
            _ => Step::Leaf(KIND_ANY.to_owned()),
        },
    }
}

fn terms_declared(parameters: Vec<Parameter>) -> Vec<Parameter> {
    assert!(parameters.len() <= 2, "an operation by property declares few parameters");

    let kept = parameters.into_iter().filter(|found| found.wire != FREE_TERMS_DECLARED);
    let mut declared: Vec<Parameter> = kept.collect();

    declared.push(Parameter {
        is_required: true,
        kind: ParameterKind::Terms,
        location: Location::Query,
        name: field_name(FREE_TERMS_FIELD),
        wire: FREE_TERMS_FIELD.to_owned(),
    });

    assert!(!declared.is_empty(), "an operation by property names its properties");

    declared
}

fn token_field_find(properties: &[(&str, &Value)]) -> Option<String> {
    assert!(!TOKEN_FIELDS.is_empty(), "a token field has a spelling");

    for field in TOKEN_FIELDS {
        if properties.iter().any(|(wire, _node)| *wire == field) {
            return Some(field.to_owned());
        }
    }

    assert!(TOKEN_FIELDS.iter().all(|field| !field.is_empty()), "a token field is spelled");

    None
}

fn unknown_location(wire: &str) -> Location {
    assert!(!wire.is_empty(), "a parameter is named");
    assert!(wire.is_empty(), "{wire} sits in a place a request cannot carry");

    Location::Query
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        Location,
        Parameter,
        ParameterKind,
        Pending,
        Registry,
        field_kind,
        field_wire,
        placeholders_check,
        placeholders_declare,
        record_is,
        step_of_type,
    };

    #[test]
    fn a_corrected_kind_reaches_only_the_field_it_names() {
        let node = json!({});
        let changelog = Pending { name: "Changelog".to_owned(), node: &node };
        let comment = Pending { name: "Comment".to_owned(), node: &node };
        let text = "crate::Text";

        assert_eq!(field_kind(&changelog, "created", text.to_owned()), "crate::Any", "the field");
        assert_eq!(field_kind(&changelog, "id", text.to_owned()), text, "its neighbour");
        assert_eq!(field_kind(&comment, "created", text.to_owned()), text, "its namesake");
    }

    #[test]
    fn a_corrected_name_reaches_only_the_field_it_names() {
        let node = json!({});
        let position = Pending { name: "DashboardGadgetPosition".to_owned(), node: &node };
        let comment = Pending { name: "Comment".to_owned(), node: &node };
        let row = "The row position of the gadget.";

        assert_eq!(field_wire(&position, row), "row", "the named field");
        assert_eq!(field_wire(&position, "title"), "title", "its neighbour");
        assert_eq!(field_wire(&comment, row), row, "its namesake");
    }

    #[test]
    fn a_declared_field_wins_over_the_undeclared_table() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);
        let node = json!({"properties": {"pageId": {"type": "integer"}}, "type": "object"});
        let source = Pending { name: "SourceDTO".to_owned(), node: &node };
        let fields = registry.fields_read(&source);
        let page = fields.iter().find(|field| field.wire == "pageId").expect("the field");

        assert_eq!(page.kind, "crate::Integer", "the specification speaks for itself");
        assert_eq!(fields.iter().filter(|field| field.wire == "pageId").count(), 1, "held once");
    }

    #[test]
    fn a_kind_peels_lists_and_maps_down_to_a_leaf() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);

        let lists =
            json!({"type": "array", "items": {"type": "array", "items": {"type": "string"}}});

        let map = json!({"type": "object", "additionalProperties": {"type": "integer"}});
        let bare = json!({});

        assert_eq!(registry.kind(&lists, "X"), "crate::List<crate::List<crate::Text>>", "nested");
        assert_eq!(registry.kind(&map, "X"), "crate::Map<crate::Integer>", "a map");
        assert_eq!(registry.kind(&bare, "X"), "crate::Any", "an empty schema");
    }

    #[test]
    fn a_path_parameter_the_template_never_names_is_refused() {
        let stray = Parameter {
            is_required: true,
            kind: ParameterKind::Text,
            location: Location::Path,
            name: "id".to_owned(),
            wire: "id".to_owned(),
        };

        assert!(placeholders_check("/issue", &[stray]).is_err(), "nothing could fill it");
    }

    #[test]
    fn a_placeholder_nobody_declared_becomes_a_required_parameter() {
        let path = "/importsource/{importSourceId}/token";
        let mut parameters: Vec<Parameter> = Vec::new();

        placeholders_declare(path, &mut parameters);

        assert_eq!(parameters.len(), 1, "the placeholder is declared");
        assert_eq!(parameters[0].wire, "importSourceId", "under its wire name");
        assert_eq!(parameters[0].location, Location::Path, "in the path");
        assert!(parameters[0].is_required, "and required");
        assert!(placeholders_check(path, &parameters).is_ok(), "the path is now satisfied");
    }

    #[test]
    fn a_reference_to_a_record_stays_a_reference_and_an_alias_is_followed() {
        let document = json!({"components": {"schemas": {
            "Names": {"type": "array", "items": {"type": "string"}},
            "User": {"type": "object", "properties": {"name": {"type": "string"}}},
        }}});

        let mut registry = Registry::new(&document);
        let record = json!({"$ref": "#/components/schemas/User"});
        let alias = json!({"$ref": "#/components/schemas/Names"});

        assert_eq!(registry.kind(&record, "X"), "crate::Object<SCHEMA::User>", "a record");
        assert_eq!(registry.kind(&alias, "X"), "crate::List<crate::Text>", "an alias");
    }

    #[test]
    fn an_inline_record_is_registered_once_under_the_name_of_where_it_sits() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);
        let inline = json!({"type": "object", "properties": {"id": {"type": "integer"}}});

        assert_eq!(registry.kind(&inline, "Board"), "crate::Object<SCHEMA::Board>", "named");
        assert_eq!(registry.kind(&inline, "Board"), "crate::Object<SCHEMA::Board>", "named again");

        registry.drain();

        assert_eq!(registry.schemas.len(), 1, "one schema, however often it was seen");
    }

    #[test]
    fn an_undeclared_field_joins_only_the_schema_it_names() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);
        let node = json!({"properties": {"type": {"type": "string"}}, "type": "object"});
        let source = Pending { name: "SourceDTO".to_owned(), node: &node };
        let comment = Pending { name: "Comment".to_owned(), node: &node };
        let held = registry.fields_read(&source);
        let other = registry.fields_read(&comment);
        let wires: Vec<&str> = held.iter().map(|field| field.wire.as_str()).collect();

        assert_eq!(wires, ["pageId", "spaceKey", "type"], "the table joins what the schema holds");
        assert_eq!(other.len(), 1, "a namesake of nothing gets nothing");
    }

    #[test]
    fn only_a_schema_with_members_is_a_record() {
        assert!(record_is(&json!({"properties": {"a": {}}})), "members");
        assert!(!record_is(&json!({"properties": {}})), "no members");
        assert!(!record_is(&json!({"type": "object"})), "no members");
        assert!(record_is(&json!({"allOf": [{}, {}]})), "a merge of two schemas");
        assert!(matches!(step_of_type(&json!({"type": "string"})), super::Step::Leaf(_)), "a leaf");
    }
}
