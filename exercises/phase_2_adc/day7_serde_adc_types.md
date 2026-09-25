# Phase 2, Day 7 -- ADC Type Definitions, Serde, and JSON Serialization

> **Time budget:** 6-8 hours of focused work.
> **Prerequisite:** Day 6 and Day 6B complete. You can manipulate
> collections with iterator chains, group data with HashMap, use
> the Entry API, transform data with closures, and chain Option
> combinators fluently.
> **Outcome:** You build the real `src/adc/` module inside Project
> Achilles. Every type you define today stays in the codebase
> permanently. Every future transformer, merge node, and report
> generator imports from this module.

> [!IMPORTANT]
> This is no longer practice. From today, you are writing
> production Achilles code. The ADC types go in `src/adc/mod.rs`.
> The unit tests go in `#[cfg(test)]` inside the same file.
> No throwaway exercise binaries.

---

## Why This Day Matters

Look at the pipeline data flow:

```
[subfinder]                     [nmap]
    |                              |
    v                              v
Vec<Host>  -----> Merge Node <----- Vec<Host>
(hostnames only)   |             (ports + services)
                   v
              Vec<Host>  (merged: hostnames + ports)
                   |
                   v
              [nuclei]
                   |
                   v
              Vec<Finding>
                   |
                   v
              [State Store / SQLite]
                   |
                   v
              [Report Generator]
```

Every arrow in that diagram is a `Vec<ADCObject>` being serialized
to JSON by the producing node and deserialized by the consuming
node. If `Host` serializes port 80 as `"port": 80` but the
downstream node expects `"port_number": 80` -- silent data loss.

Before ADC types: shell scripts pipe raw text between tools.
`nmap -oG - | grep 'open' | awk '{print $2}'`. Rename one nmap
flag and the entire pipeline breaks. No type checking. No
validation. No error until the final report is garbage.

After ADC types: every tool produces typed Rust structs. The
compiler catches mismatched fields at build time. Serde handles
serialization. Round-trip tests prove nothing is lost.

Think of Java's Jackson or Gson -- annotate a class, call
`objectMapper.writeValueAsString()`, get JSON. Serde is the
Rust equivalent, except it runs at zero cost (the serializer
is generated at compile time, not at runtime with reflection).

---

## Part 1: Serde -- The Serialization Framework

Serde is not a JSON library. It is a **framework** that separates
two concerns:

1. **Data Model:** Your Rust structs and enums define the shape
2. **Format:** JSON, YAML, TOML, MessagePack, etc. are pluggable

You derive `Serialize` and `Deserialize` on your struct ONCE.
Then you can serialize to any format without changing your struct:

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Port {
    number: u16,
    protocol: String,
    state: String,
}
```

Now this struct speaks JSON:

```rust
use serde_json;

let port = Port {
    number: 80,
    protocol: "TCP".to_string(),
    state: "Open".to_string(),
};

// Serialize: Rust struct -> JSON string
let json = serde_json::to_string(&port).unwrap();
// {"number":80,"protocol":"TCP","state":"Open"}

// Deserialize: JSON string -> Rust struct
let back: Port = serde_json::from_str(&json).unwrap();
// back.number == 80, back.protocol == "TCP"
```

### How the derive macro works (under the hood)

When you write `#[derive(Serialize)]`, the compiler generates
an `impl Serialize for Port` that visits each field and writes
it to the serializer. You never see this code. It is generated
at compile time, inlined by the optimizer, and costs zero runtime
overhead.

This is different from Java's Jackson which uses runtime
reflection to discover fields. Serde knows every field at compile
time. No reflection. No runtime cost. Zero-cost abstraction.

### Two crates, two roles

- `serde` -- the framework (traits + derive macros)
- `serde_json` -- the JSON format implementation

Your Cargo.toml already has both:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## Part 2: Deriving Serialize and Deserialize

### The five essential derives

Every ADC type needs these derives:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
```

- `Debug` -- enables `{:?}` formatting for logging
- `Clone` -- enables `.clone()` for duplicating objects
- `PartialEq` -- enables `==` comparison for round-trip testing
- `Serialize` -- enables struct -> JSON
- `Deserialize` -- enables JSON -> struct

### What fields can be serialized?

Serde can serialize any field whose type ALSO implements
Serialize/Deserialize. The standard library types all do:

- `String`, `&str`, `u8`..`u128`, `i8`..`i128`, `f32`, `f64`, `bool`
- `Vec<T>` (where T: Serialize)
- `Option<T>` (serializes as value or null/absent)
- `HashMap<String, T>` (serializes as JSON object)
- Nested structs (if they also derive Serialize)

### Nested structs

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Service {
    name: String,
    version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Port {
    number: u16,
    protocol: String,
    service: Option<Service>,   // Nested struct -- works because
                                 // Service also derives Serialize
}
```

Serialized JSON:

```json
{
  "number": 443,
  "protocol": "TCP",
  "service": {
    "name": "https",
    "version": "1.21.0"
  }
}
```

If `service` is `None`, the field appears as `null` in JSON
(unless you use `skip_serializing_if` -- covered in Part 3).

---

## Part 3: Serde Field Attributes

Serde attributes go above a struct or field to control how
serialization behaves. These are the ones you will use most.

### `#[serde(rename_all = "...")]` -- Container attribute

Controls the naming convention for ALL fields in the struct:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Finding {
    host_ref: String,        // serializes as "hostRef"
    finding_type: String,    // serializes as "findingType"
}
```

Common values:
- `"camelCase"` -- hostRef, findingType
- `"snake_case"` -- host_ref, finding_type (Rust default)
- `"SCREAMING_SNAKE_CASE"` -- HOST_REF, FINDING_TYPE
- `"kebab-case"` -- host-ref, finding-type

For Achilles, we use `snake_case` (Rust's default) so no
`rename_all` is needed on structs. But you should know it exists
because external APIs often use `camelCase`.

### `#[serde(rename = "...")]` -- Field attribute

Renames a single field:

```rust
#[derive(Serialize, Deserialize)]
struct Port {
    #[serde(rename = "port_number")]
    number: u16,             // Rust field is "number"
                             // JSON field is "port_number"
}
```

### `#[serde(skip_serializing_if = "Option::is_none")]`

Omits the field from JSON when it is `None`:

```rust
#[derive(Serialize, Deserialize)]
struct Service {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}
```

```rust
// version = Some("1.21.0") -> {"name":"https","version":"1.21.0"}
// version = None            -> {"name":"https"}
//                              (version key is completely absent)
```

Without this attribute, `None` serializes as `"version": null`.
With it, the key is simply missing from the JSON.

### `#[serde(default)]`

When deserializing, if the field is missing from JSON, use the
type's `Default` value instead of erroring:

```rust
#[derive(Serialize, Deserialize)]
struct Host {
    ip: String,
    #[serde(default)]
    ports: Vec<Port>,        // Missing in JSON -> empty Vec
    #[serde(default)]
    metadata: HashMap<String, serde_json::Value>,  // Missing -> empty map
}
```

Without `default`, a missing `ports` field causes a
deserialization error. With it, you get an empty Vec silently.

---

## Part 4: Modeling Enums with Serde

Enums are critical for ADC. Severity is not a String -- it is
one of exactly five values. Protocol is not a String -- it is
TCP, UDP, or SCTP. Using enums instead of strings means the
compiler catches typos like `"Critcal"` at build time.

### Basic enum serialization

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}
```

By default, serde serializes this as the variant name string:

```rust
let s = Severity::Critical;
let json = serde_json::to_string(&s).unwrap();
// "Critical"

let back: Severity = serde_json::from_str(&json).unwrap();
// Severity::Critical
```

### Controlling enum casing with `rename_all`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    Tcp,         // serializes as "TCP"
    Udp,         // serializes as "UDP"
    Sctp,        // serializes as "SCTP"
}
```

### Tagged enum unions -- the ADCObject pattern

When you have a Vec that can contain different ADC types (Host,
Finding, Credential), you need an enum wrapper. Serde offers
multiple tagging strategies:

**Externally tagged (default):**

```rust
#[derive(Serialize, Deserialize)]
enum ADCObject {
    Host(Host),
    Finding(Finding),
}
// JSON: {"Host": { ... }}
```

**Adjacently tagged (recommended for ADC):**

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ADCObject {
    Host(Host),
    Finding(Finding),
}
// JSON: {"type": "Host", "data": { ... }}
```

The `type` field tells the deserializer which variant to use.
The `data` field contains the actual struct. This is the pattern
Achilles uses because:

1. The type is always a top-level field -- easy to inspect
   without deserializing the full object
2. Tools can filter by type ("give me all Findings") by
   checking the `type` field alone
3. The state store (SQLite) can index on `type`

---

## Part 5: The ADC Type Hierarchy -- Network Entities

These are the core types from section 4.2 of the architecture doc.

### SourceInfo

Every ADC object tracks where it came from:

```rust
pub struct SourceInfo {
    pub tool: String,             // "nmap", "subfinder", "nuclei"
    pub tool_version: String,     // "7.93"
    pub timestamp: i64,           // Unix epoch milliseconds
    pub node_id: String,          // Workflow node that produced this
}
```

### IpAddress and IpVersion

```rust
pub enum IpVersion {
    V4,
    V6,
}

pub struct IpAddress {
    pub address: String,          // "10.0.0.1" or "::1"
    pub version: IpVersion,
    pub ptr: Option<String>,      // Reverse DNS (skip if None)
}
```

### Port and Service

```rust
pub enum Protocol {              // Serialize as "TCP", "UDP", "SCTP"
    Tcp,
    Udp,
    Sctp,
}

pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,                // nmap UDP scan ambiguity
    ClosedFiltered,              // nmap IP ID idle scan
}

pub struct Service {
    pub name: String,                 // "https"
    pub product: Option<String>,      // "nginx" (skip if None)
    pub version: Option<String>,      // "1.21.0" (skip if None)
    pub banner: Option<String>,       // Raw banner (skip if None)
    pub cpe: Vec<String>,            // default empty vec
}

pub struct Port {
    pub number: u16,
    pub protocol: Protocol,
    pub state: PortState,
    pub service: Option<Service>,    // skip if None
    pub source: SourceInfo,
}
```

### Host -- The Central Entity

```rust
pub struct Host {
    pub id: String,
    pub ip_addresses: Vec<IpAddress>,
    pub hostnames: Vec<String>,
    pub ports: Vec<Port>,            // default empty vec
    pub source: SourceInfo,
    pub metadata: HashMap<String, serde_json::Value>,  // default empty map
}
```

Host is the **central entity** in Achilles. Everything ties back
to it. A Finding references a Host by `host_ref`. A Port belongs
to a Host. A URL was discovered on a Host.

---

## Part 6: The ADC Type Hierarchy -- Web and Security Entities

### URL, HTTPResponse, and Technology

```rust
pub struct HTTPResponse {
    pub status_code: u16,
    pub content_type: Option<String>,   // skip if None
    pub title: Option<String>,          // skip if None
    pub headers: HashMap<String, String>,  // default empty map
}

pub struct Technology {
    pub name: String,                 // "nginx"
    pub version: Option<String>,      // skip if None
    pub category: String,             // "Web Server"
    pub confidence: u8,               // 0-100
}

pub struct URL {
    pub id: String,
    pub full: String,                 // "https://api.target.com:8443/login"
    pub scheme: String,               // "https"
    pub host: String,                 // "api.target.com"
    pub port: Option<u16>,            // skip if None
    pub path: String,                 // "/login"
    pub query: Option<String>,        // skip if None
    pub response: Option<HTTPResponse>,  // skip if None
    pub technologies: Vec<Technology>,   // default empty vec
    pub source: SourceInfo,
}
```

### Finding, Evidence, and FindingType

```rust
pub enum FindingType {
    Vulnerability,
    Misconfiguration,
    InformationDisclosure,
    DefaultCredential,
    WeakCryptography,
    MissingHeader,
    ExposedService,
}

pub struct Evidence {
    pub request: Option<String>,     // skip if None
    pub response: Option<String>,    // skip if None
    pub matched_at: Option<String>,  // skip if None
    pub raw_output: Option<String>,  // skip if None
}

pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub host_ref: String,
    pub finding_type: FindingType,
    pub cve: Vec<String>,           // default empty vec
    pub cwe: Vec<String>,           // default empty vec
    pub cvss: Option<f32>,          // skip if None
    pub evidence: Evidence,
    pub remediation: Option<String>,  // skip if None
    pub source: SourceInfo,
}
```

### DNSRecord and DNSType

```rust
pub enum DNSType {
    A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV, CAA,
}

pub struct DNSRecord {
    pub id: String,
    pub domain: String,               // "api.target.com"
    pub record_type: DNSType,
    pub value: String,                // "93.184.216.34"
    pub ttl: Option<u32>,            // skip if None
    pub source: SourceInfo,
}
```

---

## Part 7: Sensitive Data -- Credential and Redaction

The `Credential` type is special because it can contain
plaintext passwords. Section 7.5 of the architecture doc says:

- `plaintext_password` is populated ONLY when sqlmap extracts
  a plaintext value
- It is NEVER logged to the audit trail
- It is NEVER included in reports without explicit approval
- It MUST be behind `Option<String>` so it can be omitted

```rust
pub enum CredentialType {
    Password, Hash, SSHKey, APIKey, Token, Certificate,
}

pub struct Credential {
    pub id: String,
    pub credential_type: CredentialType,
    pub username: Option<String>,           // skip if None
    pub plaintext_password: Option<String>, // skip if None -- CRITICAL
    pub password_hash: Option<String>,      // skip if None
    pub host_ref: String,
    pub service: Option<String>,            // skip if None
    pub source: SourceInfo,
}
```

When `plaintext_password` is `None` and `skip_serializing_if`
is set, the key is **completely absent** from JSON output.
Not null -- absent. Defense in depth.

---

## Part 8: The Unified ADCObject Enum

The state store and audit log need to hold mixed types in one
collection. A single scan produces Hosts, Findings, Credentials,
and DNSRecords. They all go into one `Vec<ADCObject>`:

```rust
#[serde(tag = "type", content = "data")]
pub enum ADCObject {
    Host(Host),
    URL(URL),
    Finding(Finding),
    Credential(Credential),
    DNSRecord(DNSRecord),
}
```

### Filtering by type

Because the enum wraps each type, you can use match or
iterator methods to extract specific types:

```rust
let findings: Vec<&Finding> = objects.iter()
    .filter_map(|obj| match obj {
        ADCObject::Finding(f) => Some(f),
        _ => None,
    })
    .collect();
```

This pattern is how the report generator extracts findings
from the mixed-type state store.

---

## Part 9: Round-Trip Testing

The critical invariant for ADC types: **serialize then
deserialize must produce the exact same object.**

```rust
let original = Host { /* ... */ };
let json = serde_json::to_string(&original).unwrap();
let restored: Host = serde_json::from_str(&json).unwrap();
assert_eq!(original, restored);
```

If this assertion fails, data is being lost in the pipeline.
This is why `PartialEq` is in every derive list.

---

## Part 10: Build the ADC Module

> [!IMPORTANT]
> This is real Achilles code. Everything you write here stays
> in the codebase permanently. No throwaway exercise binaries.

### Step 1: Create the module structure

```
src/
├── main.rs
├── lib.rs           (NEW -- create this)
└── adc/
    └── mod.rs       (NEW -- all ADC types go here)
```

Create `src/lib.rs`:
```rust
pub mod adc;
```

Create `src/adc/mod.rs` -- this is where you write everything.

### Step 2: Define all enums

In `src/adc/mod.rs`, define these enums with appropriate derives
and serde attributes. Refer to Parts 4-7 for the fields:

1. `Severity` -- Info, Low, Medium, High, Critical
2. `Protocol` -- Tcp, Udp, Sctp (serialize as UPPERCASE)
3. `PortState` -- Open, Closed, Filtered, OpenFiltered, ClosedFiltered
4. `IpVersion` -- V4, V6
5. `FindingType` -- Vulnerability, Misconfiguration, InformationDisclosure,
   DefaultCredential, WeakCryptography, MissingHeader, ExposedService
6. `CredentialType` -- Password, Hash, SSHKey, APIKey, Token, Certificate
7. `DNSType` -- A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV, CAA

### Step 3: Define all structs

Define these structs with proper derives, `skip_serializing_if`
on all `Option` fields, and `#[serde(default)]` on all `Vec`
and `HashMap` fields. Refer to Parts 5-7 for exact field lists:

1. `SourceInfo` -- (tool, tool_version, timestamp, node_id)
2. `IpAddress` -- (address, version, ptr)
3. `Service` -- (name, product, version, banner, cpe)
4. `Port` -- (number, protocol, state, service, source)
5. `Host` -- (id, ip_addresses, hostnames, ports, source, metadata)
6. `HTTPResponse` -- (status_code, content_type, title, headers)
7. `Technology` -- (name, version, category, confidence)
8. `URL` -- (id, full, scheme, host, port, path, query, response, technologies, source)
9. `Evidence` -- (request, response, matched_at, raw_output)
10. `Finding` -- (id, title, description, severity, host_ref, finding_type, cve, cwe, cvss, evidence, remediation, source)
11. `DNSRecord` -- (id, domain, record_type, value, ttl, source)
12. `Credential` -- (id, credential_type, username, plaintext_password, password_hash, host_ref, service, source)

### Step 4: Define the ADCObject enum

```rust
#[serde(tag = "type", content = "data")]
pub enum ADCObject {
    Host(Host),
    URL(URL),
    Finding(Finding),
    Credential(Credential),
    DNSRecord(DNSRecord),
}
```

### Step 5: Write unit tests

At the bottom of `src/adc/mod.rs`, add a `#[cfg(test)]` module
with these tests:

1. **test_severity_roundtrip** -- Create each Severity variant,
   serialize to JSON, deserialize back, assert_eq

2. **test_protocol_uppercase** -- Serialize `Protocol::Tcp`,
   verify the JSON string is `"TCP"` (not `"Tcp"`)

3. **test_host_roundtrip** -- Create a Host with:
   - One IPv4 address
   - Two hostnames
   - Three ports (22/ssh, 80/http, 443/https)
   - Serialize to JSON, deserialize back, assert_eq

4. **test_finding_roundtrip** -- Create a Finding with
   severity Critical, one CVE, evidence with request.
   Serialize, deserialize, assert_eq

5. **test_credential_redaction** -- Create a Credential with
   `plaintext_password: None`. Serialize to JSON. Assert the
   JSON string does NOT contain `"plaintext_password"`

6. **test_adc_object_mixed** -- Create a `Vec<ADCObject>` with
   one Host, one Finding, one Credential. Serialize the Vec,
   deserialize back, assert_eq. Then use `filter_map` to
   extract only Findings and verify count == 1

7. **test_optional_fields_absent** -- Create a Service with
   `product: None, version: None`. Serialize. Assert the JSON
   does NOT contain `"product"` or `"version"` keys

Run all tests with: `cargo test`

---

## Part 11: Key Concepts to Internalize

**1. What is Serde?**
A zero-cost serialization framework. It separates the data model
(your structs) from the format (JSON, YAML, etc.). Derive macros
generate all serialization code at compile time.

**2. What does `#[derive(Serialize, Deserialize)]` do?**
Generates `impl Serialize` and `impl Deserialize` for your struct
at compile time. No runtime reflection.

**3. Why enums instead of strings for Severity/Protocol?**
The compiler catches typos. `Severity::Critcal` does not compile.
`"Critcal"` compiles fine and silently produces wrong results.

**4. What is `skip_serializing_if`?**
Omits the field from output when a condition is true.
`Option::is_none` makes None fields completely absent from JSON.

**5. What is `#[serde(default)]`?**
Uses the type's Default value when a field is missing during
deserialization. `Vec::default()` = empty vec.

**6. What is `#[serde(rename_all = "...")]`?**
Controls naming convention for all fields/variants.
`"UPPERCASE"` on Protocol makes Tcp serialize as "TCP".

**7. What is a round-trip test?**
Serialize -> deserialize -> assert_eq. Proves zero data loss.

**8. What is the ADCObject enum?**
A tagged union wrapping all ADC types. Allows heterogeneous
collections. The `type` tag drives deserialization.

**9. Why `PartialEq` in every derive?**
Enables `==` and `assert_eq!`. Without it, round-trip testing
is impossible.

**10. What is `serde_json::Value`?**
A catch-all type for arbitrary JSON. Used in the metadata field
to store tool-specific data that does not fit the canonical schema.

**11. Why are ADC types tool-agnostic?**
There is no `NmapHost` or `NucleiVulnerability`. There is `Host`
and `Finding`. The transformer converts tool-specific output into
the universal ADC type.

**12. What is `to_string` vs `to_string_pretty`?**
`to_string` = compact JSON (production). `to_string_pretty` =
indented JSON (debugging).

---

## Completion Checklist

- [ ] Read and understand Parts 1-9
      (Serde framework, derives, field attributes, enum serialization,
      ADC type hierarchy, sensitive data, ADCObject, round-trip testing)
- [ ] Create `src/lib.rs` with `pub mod adc;`
- [ ] Create `src/adc/mod.rs` with all 7 enums
- [ ] Define all 12 structs with proper serde attributes
- [ ] Define the `ADCObject` tagged enum
- [ ] Write and pass `test_severity_roundtrip`
- [ ] Write and pass `test_protocol_uppercase`
- [ ] Write and pass `test_host_roundtrip`
- [ ] Write and pass `test_finding_roundtrip`
- [ ] Write and pass `test_credential_redaction`
- [ ] Write and pass `test_adc_object_mixed`
- [ ] Write and pass `test_optional_fields_absent`
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo fmt` on all files
- [ ] Run `cargo test` -- all 7 tests pass
- [ ] Can explain why skip_serializing_if matters for credentials
- [ ] Can explain the ADCObject tagged union pattern

---

## Lesson

> The shell script pipes raw text: `nmap -oG - | grep open | cut -d'/' -f1`.
> Rename one column, and everything downstream silently breaks.
> No error. No warning. Just wrong data in the final report.
>
> Achilles does not pipe text. It serializes typed contracts:
>
>   let json = serde_json::to_string(&host)?;
>   let restored: Host = serde_json::from_str(&json)?;
>   assert_eq!(host, restored);
>
> Three lines. Serialize. Deserialize. Prove equality.
> The compiler verifies every field. Serde generates the code.
> The round-trip test catches every regression.
>
> Typed contracts are not overhead.
> They are the immune system of the pipeline.

---

**Next: Day 8 -- JSON Schema Validation and the Nmap XML Transformer ->**
