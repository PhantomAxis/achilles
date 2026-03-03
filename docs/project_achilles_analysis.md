# Project Achilles: The Offensive Security Orchestration Engine

## Complete Architectural Analysis, Issue Rectification & Masterplan

> *"The goal is not to build another automation wrapper. It is to build the lingua franca that security tools have never had — and the execution engine that speaks it natively."*

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Core Philosophy Deep-Dive](#2-core-philosophy-deep-dive)
3. [Critical Issues & Rectification Strategies](#3-critical-issues--rectification-strategies)
4. [The Achilles Data Contract (The Open Schema Standard)](#4-the-achilles-data-contract)
5. [The Workflow Engine](#5-the-workflow-engine)
6. [The Node Library Architecture](#6-the-node-library-architecture)
7. [Secrets & Security Architecture](#7-secrets--security-architecture)
8. [CLI Design & Workflow Portability](#8-cli-design--workflow-portability)
9. [Technology Stack & Infrastructure](#9-technology-stack--infrastructure)
10. [Phased Roadmap](#10-phased-roadmap)
11. [Competitive Landscape](#11-competitive-landscape)
12. [Success Metrics & KPIs](#12-success-metrics--kpis)
13. [Conclusion](#13-conclusion)

---

## 1. Executive Summary

Project Achilles is a **CLI-first security automation and orchestration platform** that treats offensive security workflows as first-class, executable, portable programs. Where today's penetration testers wire tools together with brittle bash scripts, copy-paste terminal output between tools, and lose hours to format incompatibilities — Achilles introduces a **typed data contract**, a **DAG-based execution engine**, and a **scope-enforced runtime** that makes multi-tool security workflows reproducible, shareable, and safe.

The project's deepest contribution is not the engine itself — it is the **Achilles Data Contract (ADC)**, an open schema standard for security tool I/O. No such standard exists today. Subfinder emits newline-delimited domains. Nmap emits XML (or grepable text, or JSON, depending on flags). Nuclei emits JSONL with its own field names. Ffuf emits JSON with yet another structure. Every security professional who chains these tools writes their own `jq`, `awk`, and `sed` glue code — and that glue code is fragile, untested, and non-portable.

Achilles declares: **every datum that flows between security tools is a typed object with a known schema.** A host is a `Host`. A port is a `Port`. A vulnerability is a `Finding`. Every tool node either emits these objects natively or declares a `Transformer` that bridges its raw output to the contract. This is the competitive moat — the thing nobody else has built.

### The 10/10 Vision

| Dimension | Current State | 10/10 Target |
|---|---|---|
| **Tool Integration** | Bash pipes, manual copy-paste, custom `jq` filters | Typed nodes with schema-validated I/O and auto-transformation |
| **Data Normalization** | Every tool speaks its own language; no standard exists | Open Achilles Data Contract — typed objects: Host, Port, Finding, Credential, URL |
| **Workflow Portability** | Scripts hardcoded to one operator's machine/OS/tool versions | YAML workflow files — version-controlled, shareable, executable anywhere |
| **Scope Enforcement** | Honor system; operator discipline only | Engine-level enforcement — Achilles refuses out-of-scope targets before any node executes |
| **Approval Gates** | None — scripts run everything unsupervised | Human-in-the-loop approval nodes with Slack/Discord/webhook notification |
| **Security Posture** | API keys in plaintext env vars; no audit trail | Encrypted secrets vault, sandboxed custom scripts, immutable audit log |

### The Fundamental Thesis

```
Offensive_Security_Velocity ∝ Schema_Conformance × Automation_Trust
```

The speed and reliability of a penetration testing engagement is **directly proportional** to how well tools communicate (schema conformance) and how much the operator trusts the automation to act correctly within scope (automation trust). Bash scripts maximize neither. Achilles maximizes both.

### The Achilles Principle

> **If a tool's output cannot be parsed into a typed object with known semantics, it is not integrated — it is merely invoked.**

The difference between "running nmap inside a script" and "having an nmap node in Achilles" is that the latter **understands** the output. It knows that `443/tcp` is a `Port` object with `number=443`, `protocol=TCP`, `state=OPEN`, `service=Service(name="https", product="nginx", version="1.21.0")`. This understanding is what enables downstream nodes to operate without custom glue code.

---

## 2. Core Philosophy Deep-Dive

### 2.1 The Broken State of Security Automation

Ask a senior penetration tester how they run a bug bounty workflow today. You will hear some variation of:

```
Step 1:  Run subfinder on the target domain
Step 2:  Pipe the subdomains into httpx to check which are alive
Step 3:  Copy the alive hosts into a file
Step 4:  Run nmap against each alive host
Step 5:  Parse the nmap XML with a custom script to find open web ports
Step 6:  Feed those URLs into nuclei for vulnerability scanning
Step 7:  Take the high-severity nuclei findings and run targeted ffuf or sqlmap
Step 8:  Manually compile results into a report
```

This workflow is executed by virtually every professional in the industry. And it is broken at **every seam**:

| Seam | What Breaks | How Often |
|---|---|---|
| subfinder → httpx | Subfinder emits bare domains; httpx wants `http://` prefixed URLs. Operator writes `sed 's/^/https:\/\//'` | Every time |
| httpx → nmap | httpx outputs `https://sub.target.com`; nmap wants `sub.target.com` without the scheme. Operator writes `cut -d'/' -f3` | Every time |
| nmap → nuclei | nmap emits XML; nuclei wants a list of URLs (`host:port`). Operator writes a 30-line Python script or complex `xsltproc` pipeline | Every time |
| nuclei → sqlmap | nuclei identifies a SQLi candidate URL; sqlmap needs the exact URL with injectable parameter. Operator manually copies it | Every time |
| Any step → Report | Every tool has different output format. Operator manually aggregates into Markdown/PDF | Every time |

**The critical observation:** None of these failures are about the tools themselves. Subfinder is excellent at subdomain enumeration. Nmap is excellent at port scanning. The failure is **between** the tools — at the data transformation layer that doesn't exist.

### 2.2 Why Existing Solutions Fail

#### Bash/Makefile Scripts

The most common "solution." Every senior pentester has a `recon.sh` they've accumulated over years.

**Why it fails:**
- **Brittle parsing:** `grep`, `awk`, `sed`, `cut` chains break when tool output format changes (nmap 7.80 vs 7.93 XML differences)
- **No error handling:** If nuclei crashes mid-scan, the script continues with partial data — or worse, silently drops results
- **No scope enforcement:** The script will happily scan any target you give it, including out-of-scope assets
- **Not portable:** Hardcoded paths (`/usr/bin/nmap`), OS-specific commands, version-pinned assumptions
- **No state:** If the script fails at step 5, you re-run steps 1-4 (or manually fiddle with intermediate files)
- **No approval gates:** Everything runs unsupervised. There is no "pause and ask the operator before running sqlmap against production"

#### Shuffle SOAR

An open-source SOAR (Security Orchestration, Automation, and Response) platform with a GUI workflow builder.

**Why it fails for offensive security:**
- **GUI-first:** You must use a web browser to build workflows. Pentesters live in the terminal. Building a recon workflow by dragging boxes is slower than writing the bash script
- **No data contract:** Tool integration is done through "apps" that are essentially API wrappers. The data between nodes is untyped JSON — you still write custom transformation logic per connection
- **Designed for defensive SOC workflows:** Trigger on SIEM alert → enrich with VirusTotal → create Jira ticket. Not designed for multi-stage offensive pipelines
- **No scope enforcement:** No concept of "target scope" at the engine level
- **Heavy infrastructure:** Requires Docker, a web server, a database. Overkill for a single operator running recon

#### n8n / Tines (Generic Automation)

General-purpose workflow automation platforms. Can technically orchestrate security tools.

**Why they fail:**
- **No security domain knowledge:** They don't understand what a "host" or "vulnerability" is. Every security concept must be modeled from scratch as generic JSON
- **No scope enforcement:** These tools have no concept of "this target is in scope" vs "this target is out of scope"
- **Cloud-first:** Tines is SaaS. n8n can self-host but is designed for cloud deployment. Pentesters need tools that work on an airplane, behind a VPN, on a locked-down assessment laptop
- **Expensive:** Tines pricing starts at enterprise level. n8n's free tier has limitations
- **No CLI:** Everything is GUI. No `n8n run workflow.json` from the command line

#### Nuclei Templates (Tool-Specific)

Nuclei has its own template system for vulnerability checks. Excellent within its scope.

**Why it's not enough:**
- **Single-tool:** Nuclei templates only control nuclei. They can't orchestrate subfinder → httpx → nmap → nuclei → sqlmap
- **No data normalization:** Nuclei outputs its own JSON format. It doesn't consume or produce a universal schema
- **Not a workflow engine:** No conditional logic, no loops, no parallel execution, no approval gates
- **Template, not pipeline:** A nuclei template is "check this one thing." An Achilles workflow is "do this entire engagement."

### 2.3 Achilles's Philosophical Answers

| Failure Point | Achilles's Answer |
|---|---|
| **Data incompatibility between tools** | The Achilles Data Contract (ADC) — typed objects with defined schemas. Every tool node emits ADC objects or declares a transformer |
| **Brittle parsing** | Transformers are versioned. When nmap changes its XML format, the nmap node's transformer is updated — not every script in the world |
| **No error handling** | The workflow engine has first-class error handling: retry, skip, abort, fallback nodes. State is checkpointed |
| **No scope enforcement** | Scope is declared in the workflow header. The engine validates every target against scope rules before any node executes. Violations abort the workflow |
| **No approval gates** | Approval nodes pause execution and notify the operator via Slack/Discord/webhook. Execution resumes only on explicit approval |
| **GUI-first bias** | CLI is the primary interface. `achilles run workflow.yaml` is the canonical invocation. GUI is optional, layered on top |
| **Not portable** | Workflows are YAML files. Tool versions are declared. Achilles resolves tool paths at runtime, not authoring time |
| **No state management** | The engine maintains a state store. If a workflow fails at step 5, `achilles resume <run-id>` picks up from the last successful checkpoint |
| **No audit trail** | Every node execution is logged: timestamp, target, tool, arguments, exit code, output hash. The audit log is append-only and tamper-evident |

### 2.4 The Three Pillars of Achilles

| Pillar | Description | Implementation |
|---|---|---|
| **Schema Sovereignty** | All data between nodes conforms to the Achilles Data Contract — or is explicitly transformed | ADC typed objects + Transformer nodes |
| **Scope Discipline** | The engine enforces target scope at runtime — no node can operate on out-of-scope targets | Scope Enforcement Engine in the runtime |
| **Operator Trust** | The operator can trust Achilles to do what the workflow says, pause when it should, and never exceed scope | Approval nodes + Audit log + Sandbox |

---

## 3. Critical Issues & Rectification Strategies

### 3.1 Issue Matrix

| # | Issue | Severity | Category | Rectification Strategy |
|---|---|---|---|---|
| I-01 | **Data Normalization Between Tools** | 🔴 Critical | Architecture | Achilles Data Contract with typed objects and versioned transformers |
| I-02 | **Sandbox Security for Custom Script Nodes** | 🔴 Critical | Security | Wasm-based sandbox (Wasmtime) with capability-based permissions |
| I-03 | **Tool Version Drift** | 🟡 High | Compatibility | Versioned transformer registry with multi-version support per tool |
| I-04 | **Scope Enforcement** | 🔴 Critical | Legal/Ethical | Engine-level scope validator with CIDR, domain, and regex rules |
| I-05 | **State Management Across Long-Running Workflows** | 🟡 High | Architecture | Checkpoint-based persistence with resumable execution |
| I-06 | **Parallel Execution & Race Conditions** | 🟡 High | Architecture | DAG-based scheduler with typed channels and ownership semantics |
| I-07 | **Approval Node Notification Reliability** | 🟠 Medium | Operations | Multi-channel delivery with escalation, timeout, and dead-letter handling |
| I-08 | **Secrets Management** | 🔴 Critical | Security | Encrypted vault with runtime-only injection; never serialized to logs or state |
| I-09 | **Workflow Portability Across OS Environments** | 🟠 Medium | Compatibility | Tool resolution layer with platform-aware path discovery |
| I-10 | **Adversarial Workflow Files** | 🟡 High | Security | Schema validation, static analysis, capability declarations, and sandboxing |
| I-11 | **Audit Trail Integrity** | 🟠 Medium | Compliance | Append-only log with cryptographic chaining (hash chain) |
| I-12 | **Resource Exhaustion from Runaway Nodes** | 🟠 Medium | Operations | Per-node resource limits (CPU, memory, time, network) enforced by the runtime |

---

### 3.2 I-01: Data Normalization Between Tools (CRITICAL)

**The Problem:**

This is the **single hardest problem** in security automation and the reason no existing tool has solved it comprehensively. Consider the output of five common security tools for the same target:

```
┌────────────────────────────────────────────────────────────────────┐
│                    THE DATA NORMALIZATION PROBLEM                   │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  subfinder output:         "api.target.com"                        │
│  httpx output:             {"url":"https://api.target.com",        │
│                             "status_code":200, "title":"API"}      │
│  nmap output (XML):        <host><address addr="93.184.216.34"/>   │
│                             <ports><port portid="443"              │
│                             protocol="tcp"><state state="open"/>   │
│                             <service name="https"/></port>         │
│                             </ports></host>                        │
│  nuclei output (JSONL):    {"template-id":"cve-2021-44228",        │
│                             "host":"https://api.target.com",       │
│                             "matched-at":"https://api.target.com   │
│                             /login", "severity":"critical"}        │
│  ffuf output (JSON):       {"input":{"FUZZ":"admin"},              │
│                             "status":200, "length":4821,           │
│                             "url":"https://api.target.com/admin"}  │
│                                                                    │
│  Same target. Five different schemas. Zero interoperability.       │
└────────────────────────────────────────────────────────────────────┘
```

**The Rectification: The Achilles Data Contract (ADC)**

See §4 for the full specification. In summary:
- Define canonical typed objects: `Host`, `Port`, `Service`, `URL`, `Finding`, `Credential`, `DNSRecord`
- Every tool node includes a **Transformer** that maps raw tool output → ADC objects
- Transformers are **versioned** — when a tool changes its output format, the transformer is updated, not every downstream consumer
- The ADC is published as an **open specification** — third-party tools can emit ADC natively, skipping the transformer entirely

---

### 3.3 I-02: Sandbox Security for Custom Script Nodes (CRITICAL)

**The Problem:**

Achilles allows users to write custom script nodes — arbitrary code that processes data within a workflow. This is essential for flexibility (custom parsing logic, API integrations, specialized transformations). But it is also a **massive attack surface.**

A malicious custom script could:
- Read SSH keys from `~/.ssh/`
- Exfiltrate data to an external server
- Modify other workflow files on disk
- Install a reverse shell
- Mine cryptocurrency

This isn't hypothetical. Workflow files are designed to be **shared**. A community-contributed workflow with a malicious custom script node is the #1 attack vector against Achilles users.

**The Rectification: WebAssembly Sandbox with Capability-Based Permissions**

```
┌──────────────────────────────────────────────────────────────┐
│                    CUSTOM SCRIPT SANDBOX                      │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐     │
│  │              Wasmtime Runtime                       │     │
│  │  ┌──────────────────────────────────────────────┐   │     │
│  │  │           Custom Script (WASM)               │   │     │
│  │  │                                              │   │     │
│  │  │  Allowed:                                    │   │     │
│  │  │  ├── Read ADC input objects                  │   │     │
│  │  │  ├── Emit ADC output objects                 │   │     │
│  │  │  ├── Call Achilles SDK functions              │   │     │
│  │  │  ├── Log messages                            │   │     │
│  │  │  └── DNS resolution (if declared)            │   │     │
│  │  │                                              │   │     │
│  │  │  Blocked:                                    │   │     │
│  │  │  ├── Filesystem access (outside /tmp/node)   │   │     │
│  │  │  ├── Network access (unless declared)        │   │     │
│  │  │  ├── Process spawning                        │   │     │
│  │  │  ├── Environment variable reading            │   │     │
│  │  │  └── System calls (direct)                   │   │     │
│  │  └──────────────────────────────────────────────┘   │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                              │
│  Capabilities declared in workflow YAML:                     │
│    capabilities:                                             │
│      - network: {allow: ["api.shodan.io"]}                   │
│      - filesystem: {allow: ["/tmp/achilles/node-12"]}        │
│      - time_limit: 60s                                       │
│      - memory_limit: 256MB                                   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Why WebAssembly (Wasmtime):**
- **Language-agnostic:** Custom scripts can be written in Rust, Go, C, AssemblyScript, or any language that compiles to WASM
- **True sandbox:** WASM has no ambient authority. It cannot access the filesystem, network, or system calls unless explicitly granted via WASI capabilities
- **Near-native speed:** WASM runs at ~90% of native speed. No Python interpreter overhead
- **Deterministic:** Same WASM module, same input → same output. Reproducible across machines
- **Auditable:** The WASM binary can be statically analyzed for imported functions — if it doesn't import `fd_read`, it provably cannot read files

**Alternative considered and rejected:** Docker containers for each custom script node. Too heavy — container startup time (1-3s) is unacceptable for nodes that may process thousands of items. WASM instantiation is sub-millisecond.

---

### 3.4 I-03: Tool Version Drift (HIGH)

**The Problem:**

Security tools change their output format between versions. Real examples:
- **Nmap 7.80 → 7.93:** XML attribute changes in service detection fields, new `<cpe>` element nesting
- **Nuclei v2 → v3:** JSON field renames (`template-id` → `template_id`), new severity levels
- **Ffuf 1.x → 2.x:** Changed JSON output structure, new fields added for redirect chains

A workflow created with nmap 7.80 breaks silently when run on a machine with nmap 7.93 — the transformer expects the old XML structure.

**The Rectification: Versioned Transformer Registry**

```rust
/// Each tool node declares which versions its transformer supports
struct ToolNode {
    name: String,
    /// Minimum and maximum supported tool versions
    version_range: VersionRange,
    /// Map of tool version → transformer implementation
    transformers: HashMap<SemverRange, Box<dyn Transformer>>,
}

impl ToolNode {
    fn get_transformer(&self, detected_version: &str) -> Result<&dyn Transformer> {
        // Find the transformer that covers this version
        for (range, transformer) in &self.transformers {
            if range.matches(detected_version) {
                return Ok(transformer.as_ref());
            }
        }
        Err(AchillesError::UnsupportedToolVersion {
            tool: self.name.clone(),
            version: detected_version.to_string(),
            supported: self.version_range.to_string(),
        })
    }
}
```

**Key Design Decisions:**
- **Auto-detection:** Achilles runs `nmap --version` before invoking the node and selects the appropriate transformer
- **Graceful degradation:** If a version is not in the registry, Achilles attempts the latest transformer with a warning — many version bumps don't change output format
- **Community updates:** Transformer version coverage is a contribution vector — community members can add support for new tool versions without touching core engine code

---

### 3.5 I-04: Scope Enforcement (CRITICAL)

**The Problem:**

This is the **legal and ethical critical path.** In an authorized penetration test, the scope defines exactly which targets may be tested. Scanning out-of-scope assets is potentially illegal (Computer Fraud and Abuse Act, Computer Misuse Act, EU Directive 2013/40/EU). Today, scope enforcement is pure operator discipline — and mistakes happen.

Common scope violations in automated workflows:
- Subdomain enumeration discovers `legacy.target.com` which resolves to a shared hosting IP owned by a third party
- A wildcard DNS entry causes `*.target.com` to resolve — including `notmine.target.com` which belongs to a different organization
- An IP range scope of `10.0.0.0/24` is accidentally expanded to `10.0.0.0/16` in a bash variable
- Nuclei follows a redirect from an in-scope URL to an out-of-scope domain and scans it

**The Rectification: Engine-Level Scope Validator**

```
┌─────────────────────────────────────────────────────────────┐
│                   SCOPE ENFORCEMENT ENGINE                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Workflow Header:                                           │
│  ┌────────────────────────────────────────────────────┐     │
│  │  scope:                                            │     │
│  │    domains:                                        │     │
│  │      include:                                      │     │
│  │        - "*.target.com"                            │     │
│  │        - "target.io"                               │     │
│  │      exclude:                                      │     │
│  │        - "mail.target.com"     # Shared GSuite     │     │
│  │        - "vpn.target.com"      # Production VPN    │     │
│  │    ips:                                            │     │
│  │      include:                                      │     │
│  │        - "10.0.0.0/24"                             │     │
│  │        - "192.168.1.100"                           │     │
│  │      exclude:                                      │     │
│  │        - "10.0.0.1"            # Gateway           │     │
│  │    ports:                                          │     │
│  │      exclude:                                      │     │
│  │        - 445                   # No SMB testing    │     │
│  │    protocols:                                      │     │
│  │      allow:                                        │     │
│  │        - tcp                                       │     │
│  │        - udp                                       │     │
│  └────────────────────────────────────────────────────┘     │
│                                                             │
│  Enforcement Points:                                        │
│  ┌────────────────────────────────────────────────────┐     │
│  │  1. Pre-execution validation (before any node)     │     │
│  │  2. Per-node input validation (before each node)   │     │
│  │  3. Runtime DNS resolution check (resolve, verify) │     │
│  │  4. Output filtering (remove out-of-scope results) │     │
│  └────────────────────────────────────────────────────┘     │
│                                                             │
│  On Violation:                                              │
│  ├── Log violation with full context                        │
│  ├── Skip the out-of-scope target (don't abort workflow)    │
│  ├── Emit warning to operator console                       │
│  └── If >N violations in one node → pause and ask operator  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Critical implementation detail: DNS Resolution Check**

When a workflow begins, all domain-based scope rules are resolved to IPs. During execution, when a node targets `sub.target.com`, the scope engine:
1. Resolves `sub.target.com` to its current IP
2. Checks that the IP falls within the allowed IP ranges
3. If the IP resolves to a shared hosting IP outside scope, the target is **blocked** even though the domain matches `*.target.com`

This catches the most dangerous class of scope violations — domains that resolve to infrastructure owned by third parties.

---

### 3.6 I-05: State Management Across Long-Running Workflows (HIGH)

**The Problem:**

A full bug bounty recon workflow can run for **hours or days.** If Achilles crashes at step 7 of 12, the operator should not have to re-run steps 1-6. This is worse than bash scripts — at least with bash, the intermediate files still exist on disk.

**The Rectification: Checkpoint-Based Persistence**

```
                          Workflow Execution
                               │
         ┌─────────────────────┼──────────────────────┐
         ▼                     ▼                      ▼
    ┌─────────┐          ┌─────────┐           ┌─────────┐
    │ Node A  │────────→ │ Node B  │─────────→ │ Node C  │
    └────┬────┘          └────┬────┘           └────┬────┘
         │                    │                     │
         ▼                    ▼                     ▼
    ┌─────────┐          ┌─────────┐           ┌─────────┐
    │Checkpoint│         │Checkpoint│          │Checkpoint│
    │  State   │         │  State   │          │  State   │
    │  Store   │         │  Store   │          │  Store   │
    └─────────┘          └─────────┘           └─────────┘
         │                    │                     │
         └────────────────────┴─────────────────────┘
                              │
                     ┌────────▼────────┐
                     │  SQLite State   │
                     │  Database       │
                     │  (per workflow  │
                     │   run)          │
                     └─────────────────┘
```

**State stored per checkpoint:**
- Node ID and execution status (pending / running / completed / failed / skipped)
- ADC objects emitted by the node (serialized as JSON via `serde_json`)
- Node execution metadata: start time, end time, exit code, resource usage
- Scope violations encountered
- Transformation logs

**Resume semantics:**
```bash
# Workflow fails at node 7
$ achilles run recon.yaml
[✓] Node 1: subfinder → 342 hosts
[✓] Node 2: httpx → 198 alive hosts
[✓] Node 3: nmap → 198 hosts scanned
[✗] Node 4: nuclei → FAILED (OOM killed after 45 minutes)

# Resume from last checkpoint
$ achilles resume --run-id abc123
[→] Resuming from checkpoint after Node 3
[✓] Node 4: nuclei → 47 findings (with reduced concurrency)
[✓] Node 5: report → report.pdf generated
```

---

### 3.7 I-06: Parallel Execution & Race Conditions (HIGH)

**The Problem:**

Security workflows have natural parallelism. After subdomain enumeration, you want to run httpx on all 500 subdomains concurrently, not sequentially. After identifying 50 web servers, you want to run nuclei against all 50 in parallel. But parallel execution introduces race conditions in the shared state.

**The Rectification: DAG-Based Scheduler with Ownership Semantics**

Workflows are modeled as **Directed Acyclic Graphs (DAGs)**. The scheduler guarantees:
1. A node executes only when all its input edges have data
2. Data flows through typed channels with single-writer semantics — no two nodes write to the same output channel
3. Fan-out is explicit (one node's output feeds multiple downstream nodes)
4. Fan-in uses a **Merge node** that collects results from parallel branches

```
         ┌─────────────┐
         │  subfinder   │
         └──────┬───────┘
                │ hosts: Vec<Host>
         ┌──────▼───────┐
         │    httpx     │
         └──────┬───────┘
                │ alive: Vec<URL>
       ┌────────┼────────┐        ← Fan-out (automatic)
       ▼        ▼        ▼
   ┌───────┐ ┌──────┐ ┌───────┐
   │ nmap  │ │nuclei│ │ ffuf  │   ← Parallel execution
   └───┬───┘ └──┬───┘ └───┬───┘
       │        │         │
       └────────┼─────────┘
                ▼
         ┌──────────────┐
         │   Merge      │         ← Fan-in (explicit merge node)
         └──────┬───────┘
                │ findings: Vec<Finding>
         ┌──────▼───────┐
         │   Report     │
         └──────────────┘
```

**Race condition prevention:**
- Each parallel node receives its own **copy** of the input data (copy-on-read, not shared references)
- The Merge node uses a **collect-and-wait** strategy: it waits for all upstream parallel nodes to complete before emitting its merged output
- If one parallel branch fails, the Merge node can be configured to: wait for others, abort all, or emit partial results

---

### 3.8 I-07: Approval Node Notification Reliability (MEDIUM)

**The Problem:**

Approval nodes pause workflow execution until a human approves. But if the notification doesn't reach the operator, the workflow hangs forever. SMS can fail, Slack can be muted, Discord can be offline.

**The Rectification: Multi-Channel Escalation**

```yaml
approval_node:
  message: "Nuclei found SQLi at https://api.target.com/login?id=1. Proceed with sqlmap exploitation?"
  severity: critical
  channels:
    - type: slack
      webhook: "${ACHILLES_SLACK_WEBHOOK}"
      timeout: 15m
    - type: discord
      webhook: "${ACHILLES_DISCORD_WEBHOOK}"
      timeout: 15m
    - type: cli_prompt
      # Falls back to terminal prompt if both webhooks fail/timeout
      timeout: 60m
  escalation:
    after: 30m
    action: notify_all_channels_again
  dead_letter:
    after: 120m
    action: abort_workflow
    reason: "No approval received within 2 hours"
```

**Delivery guarantees:**
- Notification is sent to **all configured channels simultaneously** (not sequentially)
- If no response after the first timeout, re-send with an escalation marker
- If no response after `dead_letter` timeout, abort the workflow with a clear reason
- Approval can be given from **any channel** — first response wins
- Each approval request has a unique token to prevent replay attacks
- Approval response is logged in the immutable audit trail

---

### 3.9 I-08: Secrets Management (CRITICAL)

**The Problem:**

Security workflows use API keys (Shodan, Censys, VirusTotal), session tokens, database credentials, and SSH keys. These secrets appear in:
- Workflow YAML files (if hardcoded — unacceptable)
- Environment variables (visible in `/proc/*/environ`)
- Tool invocation arguments (visible in `ps aux`)
- Log files (if tool output contains the secret)

**The Rectification: Encrypted Vault with Runtime-Only Injection**

```
┌──────────────────────────────────────────────────────┐
│                  SECRETS ARCHITECTURE                  │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Storage:                                            │
│  ┌────────────────────────────────────────────┐      │
│  │  ~/.achilles/vault.enc                     │      │
│  │  ├── Encrypted with AES-256-GCM            │      │
│  │  ├── Master key derived from OS keyring    │      │
│  │  │   (GNOME Keyring / macOS Keychain /     │      │
│  │  │    Windows Credential Manager)          │      │
│  │  └── Secrets stored as key-value pairs     │      │
│  └────────────────────────────────────────────┘      │
│                                                      │
│  Reference in workflow:                              │
│  ┌────────────────────────────────────────────┐      │
│  │  nodes:                                    │      │
│  │    - name: shodan_lookup                   │      │
│  │      tool: shodan                          │      │
│  │      args:                                 │      │
│  │        api_key: "${secrets.SHODAN_KEY}"     │      │
│  │                  ↑ Never stored in YAML    │      │
│  └────────────────────────────────────────────┘      │
│                                                      │
│  Injection:                                          │
│  ┌────────────────────────────────────────────┐      │
│  │  1. Workflow parser sees ${secrets.*}       │      │
│  │  2. Engine decrypts vault at runtime        │      │
│  │  3. Secret value injected into node's env   │      │
│  │  4. Node executes with secret in memory     │      │
│  │  5. Secret is zeroed from memory after node │      │
│  │  6. Audit log records "secret SHODAN_KEY    │      │
│  │     was used" — NEVER the value             │      │
│  └────────────────────────────────────────────┘      │
│                                                      │
│  Log Scrubbing:                                      │
│  ┌────────────────────────────────────────────┐      │
│  │  All node stdout/stderr passes through a   │      │
│  │  scrubber that replaces known secret values │      │
│  │  with "[REDACTED:SHODAN_KEY]"               │      │
│  └────────────────────────────────────────────┘      │
│                                                      │
└──────────────────────────────────────────────────────┘
```

---

### 3.10 I-09: Workflow Portability Across OS Environments (MEDIUM)

**The Problem:**

A workflow written on Kali Linux references `/usr/bin/nmap`. On macOS, nmap is at `/opt/homebrew/bin/nmap`. On Arch Linux, it's at `/usr/bin/nmap` but a different version. On Windows (WSL), it's at `/usr/bin/nmap` but with different capabilities.

**The Rectification: Tool Resolution Layer**

Workflows never reference absolute tool paths. Instead, they declare tool names and optional version constraints:

```yaml
nodes:
  - name: port_scan
    tool: nmap                    # Just the name — not a path
    version: ">=7.80"             # Optional version constraint
    args:
      flags: ["-sV", "-sC"]
      targets: "${input.hosts}"
```

At runtime, the **Tool Resolver** finds the tool:
1. Check `$PATH` for `nmap`
2. Verify version: `nmap --version` → parse semver
3. If version doesn't match, check Achilles's tool registry for alternative paths
4. If tool not found, emit clear error: `"Tool 'nmap' not found. Install with: sudo apt install nmap"`

---

### 3.11 I-10: Adversarial Workflow Files (HIGH)

**The Problem:**

Workflow files are designed to be shared — the "Hacker Template Library" (§8.4) is a core feature. A malicious actor could craft a workflow that:
- Runs a cryptocurrency miner as a "custom script" node
- Exfiltrates data via a `curl` node disguised as an "API enrichment" step
- Contains scope rules that appear restrictive but actually allow scanning any target
- Uses YAML anchors/aliases to hide malicious content from casual inspection

**The Rectification: Multi-Layer Validation**

```
Workflow File (.yaml)
        │
        ▼
┌───────────────────┐
│ 1. Schema         │ ← Validate against Achilles workflow JSON Schema
│    Validation     │   Reject unknown fields, invalid types, malformed scope
└───────┬───────────┘
        ▼
┌───────────────────┐
│ 2. Static         │ ← Analyze without executing:
│    Analysis       │   • Detect shell injection in node args (see below)
│                   │   • Detect YAML bombs (billion laughs)
│                   │   • Enumerate all external hosts contacted
│                   │   • Flag custom scripts that import dangerous WASI capabilities
│                   │
│  Shell Injection  │   Achilles MUST use std::process::Command's direct
│  Prevention:      │   argument API — never a shell. Command::new("nmap")
│                   │   .arg("-sV").arg(target) passes each argument
│                   │   directly to the OS via execvp. The target string
│                   │   is never interpreted by sh/bash. The moment
│                   │   anyone uses Command::new("sh").arg("-c")
│                   │   .arg(format!("nmap {}", target)), shell
│                   │   injection becomes possible. This is enforced by:
│                   │   (1) The Node trait's execute() receives args as
│                   │       Vec<String>, never as a single command string
│                   │   (2) Static analysis flags any workflow YAML
│                   │       containing shell metacharacters (; | & ` $)
│                   │       in node arguments
│                   │   (3) The subprocess runner API accepts only
│                   │       (binary: &str, args: &[&str]) — no shell mode
└───────┬───────────┘
        ▼
┌───────────────────┐
│ 3. Capability     │ ← Generate a human-readable summary:
│    Declaration    │   "This workflow will: scan *.target.com,
│                   │    use Shodan API, run 2 custom scripts with
│                   │    network access to api.shodan.io"
└───────┬───────────┘
        ▼
┌───────────────────┐
│ 4. User Consent   │ ← Operator must explicitly approve the capability
│                   │   summary before the first run
└───────────────────┘
```

This is analogous to Android's app permission model — before a workflow runs, the operator sees exactly what it will do and grants explicit consent.

---

### 3.12 I-11: Audit Trail Integrity (MEDIUM)

**The Problem:**

Penetration testing has legal requirements for provable audit trails. If Achilles runs a workflow, there must be an immutable record of exactly what was done, when, against which targets.

**The Rectification: Cryptographic Hash Chain Log**

Each log entry contains:
```
{
  "sequence": 47,
  "timestamp": "2025-03-15T14:23:01.442Z",
  "run_id": "abc-123-def",
  "node": "nmap_scan",
  "action": "execute",
  "target": "api.target.com",
  "args": ["nmap", "-sV", "-sC", "-p", "1-1000", "api.target.com"],
  "exit_code": 0,
  "output_hash": "sha256:a1b2c3d4...",
  "secrets_used": ["SHODAN_KEY"],
  "prev_hash": "sha256:e5f6a7b8...",
  "entry_hash": "sha256:9c0d1e2f..."  // hash(prev_hash + this_entry)
}
```

The `entry_hash = hash(prev_hash + serialized_entry)` creates a **hash chain** — tampering with any entry breaks the chain for all subsequent entries. This provides:
- **Tamper evidence:** Any modification to the log is detectable
- **Legal defensibility:** The audit log proves exactly what Achilles did during an authorized assessment
- **Incident response value:** If a scope violation occurs, the log shows exactly which node caused it and what data it produced

---

### 3.13 I-12: Resource Exhaustion from Runaway Nodes (MEDIUM)

**The Problem:**

A nuclei scan against 1000 hosts with all templates enabled can consume 16GB of RAM and 100% CPU for hours. An ffuf wordlist scan can generate millions of HTTP requests. Without resource limits, a single node can starve the host machine.

**The Rectification: Per-Node Resource Limits**

```yaml
nodes:
  - name: nuclei_scan
    tool: nuclei
    resources:
      memory_limit: 4GB         # Kill if exceeds
      cpu_limit: 2              # Max 2 CPU cores
      time_limit: 30m           # Kill after 30 minutes
      network:
        max_concurrent: 50      # Max 50 outbound connections
        rate_limit: 100/s       # Max 100 requests per second
      disk:
        max_output: 500MB       # Max output size
```

Enforced via:
- **Memory/CPU:** Linux cgroups v2 (the same isolation mechanism used by Docker)
- **Time:** Process-level timeout with SIGTERM → grace period → SIGKILL
- **Network:** iptables rate limiting per-node process group
- **Disk:** Output stream monitoring with early termination on limit breach

---

## 4. The Achilles Data Contract

> [!IMPORTANT]
> **This section is the architectural heart of Achilles.** The Data Contract is not a feature — it is the reason the project exists. Everything else is plumbing. Get this right, and every other problem becomes tractable. Get this wrong, and Achilles is just another scripting wrapper.

### 4.1 Design Principles

1. **Typed, not schemaless.** Every object has a defined set of fields with declared types. No `any` fields, no untyped JSON blobs
2. **Composable.** Complex objects are compositions of simpler ones. A `Host` contains `Port`s. A `Port` references a `Service`. A `Finding` references a `Host` and a `URL`
3. **Tool-agnostic.** The schema describes *security concepts*, not *tool outputs*. There is no `NmapHost` or `NucleiVulnerability` — there is `Host` and `Finding`
4. **Serializable.** Every ADC object can be serialized to JSON via `serde_json`. JSON is used for all serialization: state store persistence, audit logs, report data, and inter-node transport. MessagePack (`rmp-serde`) is a future optimization for inter-node transport once JSON is proven a bottleneck — not a launch requirement. Do not introduce `rmp-serde` until profiling demonstrates that JSON serialization is a measurable performance problem in a real workflow
5. **Extensible.** The schema supports custom metadata fields for tool-specific data that doesn't fit the canonical model — without breaking downstream consumers

### 4.2 Core Type Definitions

```
┌──────────────────────────────────────────────────────────────────┐
│                    ACHILLES DATA CONTRACT v1.0                     │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Primitive Types:                                                │
│  ├── IPv4Address    string (validated: "10.0.0.1")               │
│  ├── IPv6Address    string (validated: "::1")                    │
│  ├── DomainName     string (validated: "api.target.com")         │
│  ├── PortNumber     u16 (1-65535)                                │
│  ├── Protocol       enum { TCP, UDP, SCTP }                     │
│  ├── Severity       enum { Info, Low, Medium, High, Critical }   │
│  ├── Timestamp      i64 (Unix epoch milliseconds)                │
│  └── CIDR           string (validated: "10.0.0.0/24")           │
│                                                                  │
│  Composite Types:                                                │
│  ├── Host                                                        │
│  ├── Port                                                        │
│  ├── Service                                                     │
│  ├── URL                                                         │
│  ├── Finding                                                     │
│  ├── Credential                                                  │
│  ├── DNSRecord                                                   │
│  ├── HTTPResponse                                                │
│  └── Certificate                                                 │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

#### Host

```rust
struct Host {
    /// Unique identifier (generated by Achilles)
    id: Uuid,
    /// IP addresses (a host may have multiple)
    ip_addresses: Vec<IpAddress>,
    /// Domain names that resolve to this host
    hostnames: Vec<DomainName>,
    /// Discovered open ports
    ports: Vec<Port>,
    /// Operating system detection results
    os: Option<OSDetection>,
    /// Which tool discovered this host, and when
    source: SourceInfo,
    /// Tool-specific metadata that doesn't fit the schema
    metadata: HashMap<String, Value>,
}

struct IpAddress {
    address: String,     // "10.0.0.1" or "::1"
    version: IpVersion,  // V4 or V6
    ptr: Option<String>, // Reverse DNS
}

struct OSDetection {
    name: String,           // "Linux 5.4"
    family: String,         // "Linux"
    accuracy: u8,           // 0-100 confidence percentage
    cpe: Option<String>,    // "cpe:/o:linux:linux_kernel:5.4"
}

struct SourceInfo {
    tool: String,           // "nmap"
    tool_version: String,   // "7.93"
    timestamp: Timestamp,
    node_id: String,        // Which workflow node produced this
}
```

#### Port

```rust
struct Port {
    number: PortNumber,       // 443
    protocol: Protocol,       // TCP
    state: PortState,         // Open, Closed, Filtered
    service: Option<Service>, // What's running on this port
    source: SourceInfo,
    metadata: HashMap<String, Value>,
}

enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,   // nmap-specific: UDP scan ambiguity
    ClosedFiltered, // nmap-specific: IP ID idle scan
}
```

#### Service

```rust
struct Service {
    name: String,              // "https"
    product: Option<String>,   // "nginx"
    version: Option<String>,   // "1.21.0"
    extra_info: Option<String>,// "Ubuntu"
    cpe: Vec<String>,          // ["cpe:/a:nginx:nginx:1.21.0"]
    banner: Option<String>,    // Raw banner grab
    tunnel: Option<String>,    // "ssl" if wrapped in TLS
    metadata: HashMap<String, Value>,
}
```

#### URL

```rust
struct URL {
    id: Uuid,
    /// Full URL — "https://api.target.com:8443/login?next=/dashboard"
    full: String,
    scheme: String,             // "https"
    host: String,               // "api.target.com"
    port: Option<PortNumber>,   // 8443
    path: String,               // "/login"
    query: Option<String>,      // "next=/dashboard"
    /// HTTP response metadata (if probed)
    response: Option<HTTPResponse>,
    /// Technologies detected (Wappalyzer-style)
    technologies: Vec<Technology>,
    source: SourceInfo,
    metadata: HashMap<String, Value>,
}

struct HTTPResponse {
    status_code: u16,          // 200
    content_type: Option<String>, // "text/html"
    content_length: Option<u64>,
    title: Option<String>,     // "<title>Login</title>"
    headers: HashMap<String, String>,
    redirect_chain: Vec<String>, // Follow redirects
    body_hash: String,         // SHA256 of response body
    tls: Option<TLSInfo>,
}

struct Technology {
    name: String,              // "nginx"
    version: Option<String>,   // "1.21.0"
    category: String,          // "Web Server"
    confidence: u8,            // 0-100
}
```

#### Finding

```rust
struct Finding {
    id: Uuid,
    /// What was found
    title: String,             // "SQL Injection in login parameter"
    description: String,       // Detailed description
    severity: Severity,        // Critical
    /// Where it was found
    host: HostRef,             // Reference to the Host
    url: Option<URLRef>,       // Reference to the URL (if web-based)
    port: Option<PortRef>,     // Reference to the Port
    /// Classification
    finding_type: FindingType, // Vulnerability, Misconfiguration, InfoLeak
    /// Standard references
    cve: Vec<String>,          // ["CVE-2021-44228"]
    cwe: Vec<String>,          // ["CWE-89"]
    cvss: Option<f32>,         // 9.8
    /// Evidence
    evidence: Evidence,
    /// Remediation
    remediation: Option<String>,
    /// Which tool and template found this
    source: SourceInfo,
    template_id: Option<String>, // "nuclei:cve-2021-44228"
    metadata: HashMap<String, Value>,
}

enum FindingType {
    Vulnerability,
    Misconfiguration,
    InformationDisclosure,
    DefaultCredential,
    WeakCryptography,
    MissingHeader,
    ExposedService,
}

struct Evidence {
    /// The exact request that triggered the finding
    request: Option<String>,
    /// The exact response that confirmed the finding
    response: Option<String>,
    /// Matched pattern (regex, keyword, etc.)
    matched_at: Option<String>,
    /// Screenshot (for visual findings)
    screenshot: Option<String>, // Base64 or file path
    /// Raw tool output for this specific finding
    raw_output: Option<String>,
}
```

#### Credential

```rust
struct Credential {
    id: Uuid,
    credential_type: CredentialType,
    username: Option<String>,
    password: Option<String>,      // Only populated if explicitly exposed
    hash: Option<String>,          // "NTLM:aab3238922bcc25a..."
    token: Option<String>,         // JWT, API key, etc.
    host: HostRef,
    service: Option<String>,       // "ssh", "smb", "http-login"
    source: SourceInfo,
    // SECURITY: Credentials are NEVER written to the audit log
    // They are stored in the encrypted state only
    metadata: HashMap<String, Value>,
}

enum CredentialType {
    Password,
    Hash,
    SSHKey,
    APIKey,
    Token,
    Certificate,
}
```

#### DNSRecord

```rust
struct DNSRecord {
    id: Uuid,
    domain: DomainName,         // "api.target.com"
    record_type: DNSType,       // A, AAAA, CNAME, MX, TXT, NS, SOA
    value: String,              // "93.184.216.34"
    ttl: Option<u32>,
    source: SourceInfo,
    metadata: HashMap<String, Value>,
}

enum DNSType {
    A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV, CAA,
}
```

### 4.3 Tool-to-ADC Mapping

This table shows exactly how each major tool's output maps to ADC types:

| Tool | Raw Output Format | ADC Types Produced | Transformer Complexity |
|---|---|---|---|
| **subfinder** | Newline-delimited domains | `Vec<Host>` (hostname only, no IP) | Low — split by newline, wrap in Host |
| **httpx** | JSONL with URL, status, title, tech | `Vec<URL>` with `HTTPResponse` + `Technology` | Medium — map JSON fields to ADC fields |
| **nmap** | XML (`-oX`) | `Vec<Host>` with `Port`, `Service`, `OSDetection` | High — parse XML, map CPE, handle script output |
| **nuclei** | JSONL with template-id, severity, matched-at | `Vec<Finding>` with evidence and CVE refs | Medium — map severity levels, extract evidence |
| **ffuf** | JSON with input, status, length, URL | `Vec<URL>` with `HTTPResponse` (partial) | Low — map directly to URL objects |
| **sqlmap** | Stdout text + log files | `Vec<Finding>` + `Vec<Credential>` | High — parse unstructured text output |
| **gobuster** | Newline-delimited URLs with status codes | `Vec<URL>` with status code | Low — parse lines, construct URL objects |
| **amass** | JSONL with domain, addresses, sources | `Vec<Host>` + `Vec<DNSRecord>` | Medium — map JSON to Host and DNSRecord |
| **masscan** | Binary or JSON (`-oJ`) | `Vec<Host>` with `Port` (no service detection) | Low — map JSON fields directly |
| **hashcat** | Stdout with cracked hashes | `Vec<Credential>` (hash → password pairs) | Medium — parse output format, match to input hashes |

### 4.4 Transformer Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                      TRANSFORMER DATA FLOW                         │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Tool Process                                                      │
│  ┌──────────────────┐                                              │
│  │  $ nmap -sV -oX  │                                              │
│  │    - target.com  │                                              │
│  └────────┬─────────┘                                              │
│           │ Raw XML output                                         │
│           ▼                                                        │
│  ┌──────────────────┐                                              │
│  │  Output Capture  │  Captures stdout, stderr, exit code, and     │
│  │  Layer           │  any output files (-oX, -oJ, etc.)           │
│  └────────┬─────────┘                                              │
│           │ Raw bytes                                              │
│           ▼                                                        │
│  ┌──────────────────┐                                              │
│  │  Format Detector │  Detects: XML, JSON, JSONL, CSV, plaintext   │
│  │                  │  Uses magic bytes + content heuristics        │
│  └────────┬─────────┘                                              │
│           │ Typed raw data (XML document, JSON array, etc.)        │
│           ▼                                                        │
│  ┌──────────────────────────────────────────────────────────┐      │
│  │  Versioned Transformer                                    │      │
│  │  ┌─────────────────────────────────────────────────────┐  │      │
│  │  │  nmap_transformer_v7_93::transform(xml) -> Vec<Host>│  │      │
│  │  │                                                     │  │      │
│  │  │  1. Parse XML DOM                                   │  │      │
│  │  │  2. For each <host>:                                │  │      │
│  │  │     a. Extract IP from <address>                    │  │      │
│  │  │     b. Extract hostnames from <hostnames>           │  │      │
│  │  │     c. For each <port>:                             │  │      │
│  │  │        - Map state, protocol, portid → Port         │  │      │
│  │  │        - Map <service> → Service with CPE           │  │      │
│  │  │     d. Extract <os> → OSDetection                   │  │      │
│  │  │  3. Attach SourceInfo with tool="nmap",             │  │      │
│  │  │     version="7.93",  timestamp=now()                │  │      │
│  │  │  4. Return Vec<Host>                                │  │      │
│  │  └─────────────────────────────────────────────────────┘  │      │
│  └──────────────────────────────────────────────────────────┘      │
│           │ Vec<Host> (ADC typed objects)                           │
│           ▼                                                        │
│  ┌──────────────────┐                                              │
│  │  Schema Validator │  Validates every object against ADC schema   │
│  │                  │  Rejects malformed objects with clear errors  │
│  └────────┬─────────┘                                              │
│           │ Validated Vec<Host>                                     │
│           ▼                                                        │
│  ┌──────────────────┐                                              │
│  │  Scope Filter    │  Removes any hosts/ports that are             │
│  │                  │  out-of-scope (see I-04)                      │
│  └────────┬─────────┘                                              │
│           │ Scope-validated Vec<Host>                               │
│           ▼                                                        │
│       Next Node                                                    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

### 4.5 ADC as an Open Standard

The Achilles Data Contract should be published as an **open specification** with:
- **JSON Schema** files for each type (machine-readable, used for validation)
- **Human-readable specification** document with examples
- **Language bindings:** Rust (native), Python (PyO3), Go (cgo), TypeScript (wasm-bindgen)
- **Versioning:** Semver. Breaking changes = major version bump. New optional fields = minor version
- **License:** Apache 2.0 — permissive enough for commercial adoption

**Why open-source the schema:**

This is the single most strategic decision in the project. If the ADC becomes the standard that other tools adopt for their `--output-adc` flag, Achilles wins regardless of whether users use the Achilles engine or not. The schema becomes the lingua franca; the engine is just the first speaker. This is the same playbook as:
- **OpenAPI (Swagger):** Defines REST API schemas → every REST tool supports it
- **SARIF:** Defines static analysis results → every code scanner outputs it
- **STIX/TAXII:** Defines threat intelligence objects → every threat intel platform uses it

The ADC is the **SARIF for offensive security tooling.** No such standard exists today. This is Achilles's completely uncontested niche.

### 4.6 ADC Schema Evolution & Versioning Strategy

A typed schema that cannot evolve is a schema that will be abandoned. The ADC must change over time — tools change, new vulnerability classes emerge, the community discovers missing fields. The evolution strategy must guarantee that old workflows and transformers do not silently break when the schema updates.

**Versioning Model: Semantic Versioning (SemVer)**

```
ADC vMAJOR.MINOR.PATCH

MAJOR (1.0 → 2.0): Breaking changes. Existing transformers may not produce valid output.
MINOR (1.0 → 1.1): New optional fields or new types. All existing data remains valid.
PATCH (1.0.0 → 1.0.1): Documentation fixes, clarifications. No schema changes.
```

**What constitutes a breaking change:**

| Change | Breaking? | Example |
|---|---|---|
| Adding a new optional field to an existing type | No | Adding `Host.cloud_provider: Option<String>` |
| Adding a new ADC type | No | Adding `Certificate` type |
| Removing a field | **Yes** | Removing `Finding.cvss` |
| Renaming a field | **Yes** | `Finding.cve` → `Finding.cve_ids` |
| Changing a field's type | **Yes** | `Port.number: u16` → `Port.number: String` |
| Making an optional field required | **Yes** | `Host.os: Option<OS>` → `Host.os: OS` |
| Narrowing an enum | **Yes** | Removing `Severity::Info` |
| Widening an enum | No | Adding `Severity::None` |

**Reserved Fields (inspired by Protocol Buffers):**

Protobuf has a `reserved` keyword that prevents accidental reuse of field numbers after a field is deprecated. The ADC adopts an equivalent concept:

```json
{
  "$schema": "https://adc.achilles.dev/v1/host.schema.json",
  "type": "object",
  "properties": {
    "id": { "type": "string", "format": "uuid" },
    "ip_addresses": { "type": "array", "items": { "type": "string" } },
    "hostnames": { "type": "array", "items": { "type": "string" } },
    "ports": { "type": "array", "items": { "$ref": "port.schema.json" } }
  },
  "required": ["id", "ip_addresses", "hostnames"],
  "reserved_fields": {
    "_comment": "These field names are permanently retired. Do not reuse them.",
    "fields": ["ip", "hostname"],
    "reason": {
      "ip": "Renamed to ip_addresses in v1.0 (was singular in draft). Reserved to prevent confusion.",
      "hostname": "Renamed to hostnames in v1.0 (was singular in draft). Reserved to prevent confusion."
    }
  },
  "additionalProperties": false
}
```

The `reserved_fields` block is not enforced by standard JSON Schema validators — it is enforced by the **Achilles schema validator** (a superset of JSON Schema) and documented in the specification. When a transformer author creates a new field, the validator checks it against the reserved list and rejects collisions.

**Node Schema Version Compatibility Declaration:**

Every node declares which ADC versions it supports:

```rust
impl Node for NmapNode {
    fn adc_version_range(&self) -> VersionRange {
        // This transformer is compatible with ADC v1.0 through v1.x
        VersionRange::new(">=1.0.0 <2.0.0")
    }
}
```

At workflow validation time, the engine checks that all nodes in the DAG are compatible with the same ADC version. If node A supports ADC v1.x and node B requires ADC v2.x, the workflow is rejected with a clear error explaining the incompatibility and which node needs updating.

**Migration Guide Requirement:**

Every major version bump **must** include a migration guide documenting:
1. Every breaking change with before/after examples
2. A `achilles migrate <workflow.yaml> --from v1 --to v2` CLI command that rewrites workflow files
3. A transformer compatibility matrix showing which nodes are updated

### 4.7 ADC Adoption Strategy

The ADC's value is zero if only Achilles speaks it. An open standard requires adoption. This is a concrete plan — not aspirational goals — for getting security tool vendors and their communities to emit ADC output natively.

**Phase 1: Prove value through Achilles itself (Months 1-6)**

Before asking any tool to adopt the ADC, Achilles must prove the schema works. Ship 6 production-quality transformers (subfinder, httpx, nmap, nuclei, ffuf, sqlmap) that convert raw tool output to ADC objects. Publish the transformer code as open source. Publish the schema as a standalone repository with JSON Schema files, documentation, and language bindings.

The proof: show real-world ADC output from real tools. Not a specification document — working code.

**Phase 2: Target ProjectDiscovery tools first (Months 6-12)**

ProjectDiscovery (the organization behind subfinder, httpx, nuclei, katana, and others) is the ideal first adopter because:
- They are **community-driven** and accept contributions from external developers
- Their tools already support `--json` and `-jsonl` output — the ADC format is an incremental step, not a rewrite
- They control **multiple tools in the same pipeline** — if subfinder and httpx both emit ADC, the value proposition is immediately obvious
- Their tools are written in **Go** — the ADC Go language binding makes integration frictionless

**The technical proposal to ProjectDiscovery:**

```
Subject: RFC — Achilles Data Contract output format for PD tools

Proposal: Add a `--format adc` flag to subfinder, httpx, nuclei

What it does:
  Instead of:  subfinder -d target.com -json
            →  {"host": "api.target.com", "source": "crtsh"}

  You get:     subfinder -d target.com -format adc
            →  {"adc_version": "1.0", "type": "Host",
                "id": "uuid", "hostnames": ["api.target.com"],
                "ip_addresses": [], "source": {"tool": "subfinder", ...}}

Why:
  - Achilles users get zero-transformer integration (no parsing code needed)
  - Any tool that reads ADC can consume PD tool output without custom parsers
  - PD tools become the reference implementation of the ADC

Implementation effort:
  ~200 lines of Go per tool. The ADC Go binding handles serialization.
  We (Achilles team) will write the PRs.

License: ADC schema is Apache 2.0 — no licensing concerns.
```

**The incentive structure:**

| Actor | Incentive to adopt ADC |
|---|---|
| **Tool author** | Zero: "my tool already has JSON output, why add another format?" → overcome by writing the PR for them and showing downstream adoption |
| **Pipeline author** | High: "I can chain 5 tools without writing 5 custom parsers" → this is Achilles's primary value proposition |
| **Tool user community** | Medium: "I want my tool's output to work with Achilles and any future ADC-compatible platform" → community pull on tool authors |

The strategy is to **pull from the demand side, not push from the supply side.** Achilles users requesting `--format adc` in tool issue trackers creates organic pressure. The Achilles team writing the implementation PRs removes the effort barrier.

**Phase 3: Second wave — independent tools (Months 12-24)**

Once ProjectDiscovery tools support `--format adc`, target:
1. **Nmap** — hardest (C codebase, slow release cycle) but highest value. Propose an nmap NSE script that converts XML to ADC as a bridge.
2. **Sqlmap** — Python, active development. Propose `--output-format adc` flag.
3. **Masscan** — C, focused on speed. Propose output post-processor rather than native integration.
4. **Amass** — Go, OWASP project. Good governance for accepting new output formats.

**Success metric:** If 3 tools emit native ADC output by Month 18, the standard has crossed the adoption threshold. At that point, new tools have an incentive to support ADC because the ecosystem expects it — the same flywheel that made SARIF ubiquitous for SAST tools.

---

## 5. The Workflow Engine

### 5.1 Engine Overview

The Workflow Engine is the runtime that parses, validates, schedules, and executes workflows. It is a **DAG executor** — every workflow is a Directed Acyclic Graph of nodes, where edges represent typed data flow.

```
┌──────────────────────────────────────────────────────────────────────┐
│                      WORKFLOW ENGINE ARCHITECTURE                     │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────┐                                                    │
│  │ Workflow YAML │                                                   │
│  └──────┬───────┘                                                    │
│         ▼                                                            │
│  ┌──────────────────┐     ┌──────────────────┐                       │
│  │   YAML Parser    │────▶│  Schema Validator │                      │
│  │   + Resolver     │     │  (JSON Schema)    │                      │
│  └──────────────────┘     └────────┬──────────┘                      │
│                                    ▼                                  │
│                           ┌──────────────────┐                       │
│                           │  DAG Builder     │                       │
│                           │  (dependency     │                       │
│                           │   resolution)    │                       │
│                           └────────┬─────────┘                       │
│                                    ▼                                  │
│                           ┌──────────────────┐                       │
│                           │  Cycle Detector  │  ← Reject cyclic      │
│                           │                  │    workflows           │
│                           └────────┬─────────┘                       │
│                                    ▼                                  │
│  ┌──────────────────────────────────────────────────────────────┐    │
│  │                     EXECUTION RUNTIME                        │    │
│  │                                                              │    │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │    │
│  │  │ Scope Engine │  │ Secret Vault │  │ Tool Resolver│       │    │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │    │
│  │         │                 │                  │               │    │
│  │         └─────────────────┼──────────────────┘               │    │
│  │                           ▼                                  │    │
│  │                  ┌──────────────────┐                        │    │
│  │                  │   DAG Scheduler  │                        │    │
│  │                  │   (topological   │                        │    │
│  │                  │    execution)    │                        │    │
│  │                  └────────┬─────────┘                        │    │
│  │                           │                                  │    │
│  │            ┌──────────────┼──────────────┐                   │    │
│  │            ▼              ▼              ▼                   │    │
│  │     ┌──────────┐   ┌──────────┐   ┌──────────┐             │    │
│  │     │ Node     │   │ Node     │   │ Node     │             │    │
│  │     │ Executor │   │ Executor │   │ Executor │             │    │
│  │     │ (thread) │   │ (thread) │   │ (thread) │             │    │
│  │     └────┬─────┘   └────┬─────┘   └────┬─────┘             │    │
│  │          │              │              │                    │    │
│  │          ▼              ▼              ▼                    │    │
│  │     ┌──────────────────────────────────────┐               │    │
│  │     │         State Store (SQLite)         │               │    │
│  │     │  Checkpoints, ADC objects, logs      │               │    │
│  │     └──────────────────────────────────────┘               │    │
│  │                                                              │    │
│  └──────────────────────────────────────────────────────────────┘    │
│                           │                                          │
│                           ▼                                          │
│                  ┌──────────────────┐                                │
│                  │   Audit Logger   │                                │
│                  │   (hash chain)   │                                │
│                  └──────────────────┘                                │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 5.2 Workflow Parsing and Validation

Workflow files go through a **four-stage validation pipeline** before any execution begins:

**Stage 1 — Syntax Validation:**
Parse YAML. Reject malformed syntax, YAML bombs (billion laughs attack via anchors), and files exceeding 10MB.

**Stage 2 — Schema Validation:**
Validate against the Achilles Workflow JSON Schema:
- All required fields present (`name`, `version`, `scope`, `nodes`)
- All node types are known (registered tool nodes or logic nodes)
- All edges reference existing nodes
- All secret references (`${secrets.*}`) have valid identifiers
- Scope rules are syntactically valid (valid CIDR, valid domain patterns)

**Stage 3 — DAG Validation:**
Build the dependency graph and verify:
- **No cycles.** A workflow that feeds Node B's output back into Node A is rejected with a clear error showing the cycle
- **All inputs satisfied.** Every node's declared inputs are produced by an upstream node's declared outputs
- **Type compatibility.** If Node A emits `Vec<Host>` and Node B expects `Vec<URL>`, the edge must have a transformer — or there must be a type-compatible conversion path (Host → URL is possible if the host has web ports)

**Stage 4 — Scope Validation:**
Pre-resolve all static targets in the workflow:
- Domain names are resolved to IPs
- IPs are checked against scope rules
- If any static target is out of scope, the workflow is rejected **before execution begins**

```rust
pub fn validate_workflow(yaml: &str) -> Result<ValidatedWorkflow, Vec<ValidationError>> {
    let raw = parse_yaml(yaml)?;                     // Stage 1
    let schema_valid = validate_schema(&raw)?;       // Stage 2
    let dag = build_dag(&schema_valid)?;             // Stage 3 (cycle + type check)
    let scope_valid = validate_scope(&dag)?;         // Stage 4
    Ok(ValidatedWorkflow { dag, scope: scope_valid })
}
```

### 5.3 Sequential vs Parallel Node Execution

The DAG scheduler uses **topological sort with parallelism detection:**

1. Compute topological order of all nodes
2. Identify **independent node groups** — nodes at the same depth that have no edge between them
3. Execute each group in parallel; wait for all nodes in a group to complete before starting the next group

```
Example Workflow DAG:

    A (subfinder)
    │
    B (httpx)
    │
    ├──── C (nmap)
    │        │
    ├──── D (nuclei)        ← C, D, E are independent → parallel
    │        │
    └──── E (ffuf)
             │
          F (merge)          ← Waits for C, D, E
             │
          G (report)

Execution Schedule:
  Round 1: [A]              (sequential — one node)
  Round 2: [B]              (sequential — one node, depends on A)
  Round 3: [C, D, E]        (parallel — no dependencies between them)
  Round 4: [F]              (sequential — depends on C, D, E)
  Round 5: [G]              (sequential — depends on F)
```

### 5.4 State Management: Data Flow Between Nodes

When a node completes, its ADC output objects are:
1. **Serialized** to JSON and written to the state store (SQLite)
2. **Schema-validated** against the ADC type definitions
3. **Scope-filtered** — any out-of-scope objects are removed and logged
4. **Made available** to downstream nodes via typed channels

Downstream nodes receive data through a **typed pull interface:**

```rust
/// The API available to every node for reading input
trait NodeInput {
    /// Get all objects of type T from a specific upstream node
    fn get<T: ADCType>(&self, from_node: &str) -> Vec<T>;

    /// Get all objects of type T from all upstream nodes
    fn get_all<T: ADCType>(&self) -> Vec<T>;

    /// Get a filtered subset
    fn get_filtered<T: ADCType>(
        &self,
        from_node: &str,
        filter: impl Fn(&T) -> bool
    ) -> Vec<T>;
}

// Example: nuclei node reading hosts from nmap
fn execute(&self, input: &dyn NodeInput) -> Result<Vec<Finding>> {
    let hosts: Vec<Host> = input.get("nmap_scan");
    let urls: Vec<String> = hosts.iter()
        .flat_map(|h| h.web_urls())  // Helper: extract http(s) URLs from open ports
        .collect();
    // Run nuclei against these URLs...
}
```

### 5.5 Error Handling

Each node has a configurable **error strategy** declared in the workflow:

```yaml
nodes:
  - name: nuclei_scan
    tool: nuclei
    on_error:
      strategy: retry        # retry | skip | abort | fallback
      max_retries: 3
      retry_delay: 30s
      safe_to_retry: true    # See retry safety below
      fallback_node: basic_scan  # Only if strategy=fallback
```

| Strategy | Behavior |
|---|---|
| **retry** | Re-execute the node up to `max_retries` times with exponential backoff. If all retries fail, abort the workflow |
| **skip** | Mark the node as skipped, continue with downstream nodes. Downstream nodes that depend on this node's output receive an empty set |
| **abort** | Immediately halt the entire workflow. All running parallel nodes are sent SIGTERM. State is checkpointed for resume |
| **fallback** | Execute an alternative node (`fallback_node`) instead. The fallback node must produce the same output type |

**Retry Safety for Exploitation Nodes:**

Retrying is NOT universally safe. Re-executing sqlmap against a target that just triggered a WAF block will escalate detection and may get the operator's IP permanently banned or flagged in the client's SOC.

| Node Category | Safe to Retry | Reason |
|---|---|---|
| **Reconnaissance** (subfinder, httpx) | ✅ Always | Passive or low-impact; re-running is harmless |
| **Scanning** (nmap, nuclei) | ✅ On infrastructure errors only | Retry on timeout, OOM, tool crash. Do NOT retry if the tool exited cleanly with a non-zero code indicating a WAF/IDS response (nuclei exit code 1 with "blocked" in stderr) |
| **Exploitation** (sqlmap) | ⚠ Never auto-retry | Exploitation changes target state. A failed sqlmap run may have partially injected. Re-running is dangerous. Require `--force-rerun-node sqlmap` for explicit operator decision |
| **Logic/Merge** | ✅ Always | No external side effects |

The `safe_to_retry` field defaults to `true` for reconnaissance and logic nodes, `false` for exploitation nodes. The engine checks this field before executing a retry — if `safe_to_retry: false` and the failure was not an infrastructure error (timeout, OOM, tool-not-found), the engine treats the failure as `abort` regardless of the declared strategy.

**Checkpoint State Machine:**

Every node transitions through these states:

```
 PENDING → RUNNING → COMPLETED
                  ↘ FAILED
                  ↘ SKIPPED
```

| State | Meaning | Resume behavior |
|---|---|---|
| `PENDING` | Not yet started | Execute normally |
| `RUNNING` | Execution started but not completed | **Treated as FAILED on resume** — the engine cannot know if the node partially executed. Operator must use `--force-rerun-node <name>` to explicitly restart |
| `COMPLETED` | Finished successfully, output checkpointed | Skip on resume |
| `FAILED` | Failed with error | Re-execute according to error strategy (if retries remain) |
| `SKIPPED` | Skipped due to upstream failure or operator decision | Skip on resume |

The `RUNNING → FAILED` rule on crash is critical for exploitation nodes: if sqlmap was mid-execution when the engine crashed, it may have already sent exploitation payloads. Automatically re-running it without operator review is negligent. The `--force-rerun-node` flag forces a conscious decision.

### 5.6 The Approval Node Mechanism

Approval nodes are **first-class node types** in the DAG, not afterthoughts. They pause execution, present context to the operator, and resume only on explicit approval.

```
┌─────────────────────────────────────────────────────────────────┐
│                    APPROVAL NODE LIFECYCLE                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. Upstream node completes                                     │
│     │                                                           │
│  2. Approval node activates                                     │
│     ├── Generates approval context (what data will be acted on) │
│     ├── Computes risk summary:                                  │
│     │   "3 critical SQLi findings on production endpoints.      │
│     │    sqlmap will attempt exploitation on:"                  │
│     │   - https://api.target.com/login?id=1                     │
│     │   - https://api.target.com/search?q=test                  │
│     │   - https://api.target.com/user/profile?uid=42            │
│     ├── Sends notification to all configured channels           │
│     └── Pauses execution (state = AWAITING_APPROVAL)            │
│     │                                                           │
│  3. Operator reviews context                                    │
│     ├── APPROVE → execution resumes with full data              │
│     ├── APPROVE_PARTIAL → operator selects subset of targets    │
│     ├── REJECT → workflow continues, skipping downstream nodes  │
│     └── ABORT → workflow terminates immediately                 │
│     │                                                           │
│  4. Decision is logged in audit trail with:                     │
│     ├── Who approved (operator identity)                        │
│     ├── When (timestamp)                                        │
│     ├── What was approved (exact target list)                   │
│     └── Which channel the approval came from                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**APPROVE_PARTIAL** is a critical feature. When nuclei finds 50 SQLi candidates, the operator may want to exploit only the 3 most promising ones. The approval node allows the operator to select a subset, and only that subset flows to the downstream sqlmap node.

**Approval Timeout Model:**

The default state for an approval node is `AWAITING_APPROVAL` with **no timeout** (hibernate). The workflow pauses indefinitely until the operator responds. This is intentional: engagements run overnight, across time zones, and over weekends. A 2-hour auto-reject would break any workflow started before the operator leaves for the day.

Operators who want a deadline can set an optional `auto_reject_after` field:

```yaml
- name: exploit_approval
  type: approval
  config:
    auto_reject_after: 24h    # Optional — reject if no response in 24 hours
    # Omit this field entirely for indefinite hibernate (the default)
```

When `auto_reject_after` fires, the approval is treated as `REJECT` — downstream nodes are skipped, the workflow continues with the non-exploitation branch (if any), and the decision is logged as `auto_rejected_timeout` in the audit trail.

**Notification Model — Outbound Only:**

Achilles is a local CLI tool. It cannot receive inbound webhooks without a reverse tunnel, which introduces a dependency and an attack surface that are unacceptable for a security tool.

The correct model: Achilles sends **outbound notifications only**. The Slack/Discord webhook message includes:
1. The approval context (findings summary, targets, risk assessment)
2. The approval token: a one-time `<token>` string
3. The CLI command to execute: `achilles approve <run-id> <token>`

The operator reads the Slack message, opens any terminal with Achilles installed, and runs the approve command. The token authenticates the decision. No inbound webhook, no reverse tunnel, no exposed port.

```
Slack Message:
┌──────────────────────────────────────────────┐
│ 🔴 ACHILLES: Approval Required               │
│                                               │
│ Workflow: bug_bounty_recon                    │
│ Node: sqlmap_exploit                          │
│ 3 critical SQLi findings on 3 endpoints       │
│                                               │
│ To approve:                                   │
│ achilles approve abc123 tok_9f8a2b...         │
│                                               │
│ To reject:                                    │
│ achilles reject abc123 tok_9f8a2b...          │
└──────────────────────────────────────────────┘
```

### 5.7 Scope Enforcement at Runtime

Scope enforcement is not a single check — it is a **continuous validation** that runs at three points:

1. **Pre-workflow:** Validate all static targets in the workflow YAML
2. **Pre-node:** Before each node executes, validate its input targets against scope
3. **Post-node:** After each node produces output, filter out any out-of-scope results (handles tools that follow redirects or discover new targets)

```rust
pub struct ScopeEngine {
    rules: ScopeRules,
    /// Cache of domain → IP resolutions (refreshed every 5 minutes)
    dns_cache: DnsCache,
    /// Known out-of-scope targets (accumulated during the run)
    violations: Vec<ScopeViolation>,
}

impl ScopeEngine {
    pub fn check_target(&self, target: &Target) -> ScopeResult {
        match target {
            Target::Domain(d) => {
                // 1. Check domain against domain rules
                if self.rules.domains.is_excluded(d) {
                    return ScopeResult::Denied("Domain explicitly excluded");
                }
                if !self.rules.domains.is_included(d) {
                    return ScopeResult::Denied("Domain not in scope");
                }
                // 2. Resolve domain and check IP
                let ips = self.dns_cache.resolve(d);
                for ip in ips {
                    if !self.rules.ips.is_included(&ip) {
                        return ScopeResult::Denied(
                            "Domain resolves to out-of-scope IP"
                        );
                    }
                }
                ScopeResult::Allowed
            }
            Target::IP(ip) => {
                if !self.rules.ips.is_included(ip) {
                    return ScopeResult::Denied("IP not in scope");
                }
                ScopeResult::Allowed
            }
            Target::Port(host, port) => {
                let host_result = self.check_target(&Target::from(host));
                if host_result.is_denied() {
                    return host_result;
                }
                if self.rules.ports.is_excluded(port) {
                    return ScopeResult::Denied("Port explicitly excluded");
                }
                ScopeResult::Allowed
            }
        }
    }
}
```

#### 5.7.1 TOCTOU Mitigation: DNS Race Conditions

The three-point scope enforcement model has a Time-of-Check to Time-of-Use (TOCTOU) vulnerability in the default design: the scope engine resolves `api.target.com` to `10.0.0.5` at pre-workflow time and confirms it is in scope. Ten minutes later, when nmap actually executes, `api.target.com` may resolve to a different IP — either through DNS TTL expiry, DNS load balancing, or a deliberate DNS rebinding attack by TA-4 (Tool Output Injector).

**Mitigation: Engine-Resolved Pre-Binding**

The engine performs DNS resolution exactly once per target per workflow run. The resolved IP addresses are stored in the `ScopeEngine::dns_cache` and passed directly to tool nodes as pre-resolved targets. Tools receive IP addresses, not hostnames:

```rust
impl ScopeEngine {
    /// Resolve all targets to IPs at workflow start. Tools receive
    /// these pre-resolved IPs — they never perform their own DNS lookups.
    pub fn pre_resolve_targets(&mut self, targets: &[Target]) -> Result<ResolvedTargets> {
        let mut resolved = ResolvedTargets::new();
        for target in targets {
            match target {
                Target::Domain(d) => {
                    let ips = dns_resolve(d)?;  // Single resolution point
                    for ip in &ips {
                        if !self.rules.ips.is_included(ip) {
                            return Err(ScopeError::ResolvesToOutOfScope {
                                domain: d.clone(),
                                ip: *ip,
                            });
                        }
                    }
                    resolved.bind(d.clone(), ips);
                }
                Target::IP(ip) => resolved.bind_direct(*ip),
                _ => {}
            }
        }
        Ok(resolved)
    }
}
```

**Tool argument construction uses pre-resolved IPs:**

```rust
// Instead of: Command::new("nmap").arg("api.target.com")
// The engine passes:
let resolved_ip = scope_engine.resolved_targets.get("api.target.com")
    .expect("all targets pre-resolved at workflow start");
Command::new("nmap").arg("-sV").arg(resolved_ip.to_string())
```

This eliminates the TOCTOU window entirely: the DNS resolution that the scope engine checked is the same IP that the tool receives . No secondary lookup occurs.

**Exception: Tools that require hostnames.** Some tools (httpx, nuclei) send HTTP requests with a `Host:` header and need the original hostname for correct routing. In these cases, the engine passes both the hostname and the pre-resolved IP:

```rust
// httpx needs the hostname for the Host header, but resolves to our IP
Command::new("httpx")
    .arg("-u").arg("https://api.target.com")  // Hostname for Host header
    .arg("-resolve").arg(format!("api.target.com:{}", resolved_ip))  // Force resolution
```

The post-node scope check (point 3) remains as a safety net: if a tool follows a redirect to a new hostname that was not pre-resolved, the output is filtered and logged as a potential scope violation.

---

## 6. The Node Library Architecture

### 6.1 Node Interface Specification

Every node in Achilles — whether it wraps a security tool, performs logic, or runs a custom script — implements the same interface:

```rust
/// The universal node interface. Every node in Achilles implements this.
pub trait Node: Send + Sync {
    /// Unique identifier for this node type (e.g., "nmap", "conditional", "custom_script")
    fn type_id(&self) -> &str;

    /// Human-readable name
    fn display_name(&self) -> &str;

    /// What ADC types this node can consume
    fn input_schema(&self) -> Vec<TypeDescriptor>;

    /// What ADC types this node produces
    fn output_schema(&self) -> Vec<TypeDescriptor>;

    /// Configuration options (CLI flags, parameters)
    fn config_schema(&self) -> ConfigSchema;

    /// Validate that the node's configuration is correct before execution
    fn validate(&self, config: &NodeConfig) -> Result<(), Vec<ValidationError>>;

    /// Execute the node. Receives typed input, produces typed output.
    fn execute(
        &self,
        input: &dyn NodeInput,
        config: &NodeConfig,
        context: &ExecutionContext,  // scope engine, secrets, logger
    ) -> Result<NodeOutput, NodeError>;

    /// Supported tool versions (empty for logic nodes)
    fn supported_versions(&self) -> Option<VersionRange>;
}

/// ADCObject uses an enum rather than Box<dyn ADCType>.
/// Reason: trait objects in Rust require object-safety — no generics in
/// trait methods, no Self in return types. This creates real friction when
/// nodes want to return Vec<Host> vs Vec<Finding>. The enum approach is
/// simpler, more ergonomic, and gives exhaustive match checking (the
/// compiler tells you when a new ADC type is added but not handled).
/// The tradeoff: adding a new ADC type requires adding an enum variant
/// and updating all match arms. This is preferable to the alternative
/// (dyn Any with downcasting), which is error-prone and loses type safety.
pub enum ADCObject {
    Host(Host),
    Port(Port),
    Service(Service),
    Url(URL),
    Finding(Finding),
    Credential(Credential),
    DnsRecord(DNSRecord),
}

pub struct NodeOutput {
    /// The ADC objects produced by this node
    pub objects: Vec<ADCObject>,
    /// Execution metadata
    pub duration: Duration,
    pub exit_code: Option<i32>,
    /// Warnings (non-fatal issues)
    pub warnings: Vec<String>,
}
```

### 6.2 Built-In Tool Nodes

#### Subfinder Node

```
┌─────────────────────────────────────────────────────┐
│  Node: subfinder                                     │
├─────────────────────────────────────────────────────┤
│  Purpose: Passive subdomain enumeration              │
│                                                      │
│  Input:   Vec<DomainName> (target domains)           │
│  Output:  Vec<Host> (discovered subdomains)          │
│                                                      │
│  Config:                                             │
│    sources: [crtsh, hackertarget, ...]               │
│    threads: 10                                       │
│    timeout: 30s                                      │
│    recursive: false                                  │
│    api_keys:                                         │
│      shodan: ${secrets.SHODAN_KEY}                   │
│      censys: ${secrets.CENSYS_KEY}                   │
│                                                      │
│  Transformer:                                        │
│    Input format: newline-delimited domains           │
│    Mapping:                                          │
│      "api.target.com" → Host {                       │
│        hostnames: ["api.target.com"],                │
│        ip_addresses: [],  // not resolved yet        │
│        source: SourceInfo { tool: "subfinder" }      │
│      }                                               │
│                                                      │
│  Scope check: Each discovered subdomain is checked   │
│  against scope rules before being emitted            │
└─────────────────────────────────────────────────────┘
```

#### Nmap Node

```
┌─────────────────────────────────────────────────────┐
│  Node: nmap                                          │
├─────────────────────────────────────────────────────┤
│  Purpose: Port scanning + service detection          │
│                                                      │
│  Input:   Vec<Host> (targets to scan)                │
│  Output:  Vec<Host> (enriched with Port + Service)   │
│                                                      │
│  Config:                                             │
│    scan_type: syn | connect | udp | version          │
│    ports: "1-1000" | "top-100" | "all"               │
│    flags: ["-sV", "-sC", "-O", "--script=vuln"]      │
│    timing: T3                                        │
│    max_concurrent: 5  # hosts scanned in parallel    │
│                                                      │
│  Transformer (HIGH complexity):                      │
│    Input format: XML (-oX output)                    │
│    Mapping:                                          │
│      <host> → Host {                                 │
│        ip: from <address addr="..."/>,               │
│        hostnames: from <hostname name="..."/>,       │
│        ports: [                                      │
│          <port> → Port {                             │
│            number: portid,                           │
│            protocol: protocol,                       │
│            state: from <state state="..."/>,         │
│            service: Service {                        │
│              name, product, version, cpe             │
│              from <service .../> element             │
│            }                                         │
│          }                                           │
│        ],                                            │
│        os: from <osmatch name="..." accuracy="..."/> │
│      }                                               │
│                                                      │
│  Versions supported: 7.80, 7.91, 7.92, 7.93, 7.94   │
│  Version auto-detected via: nmap --version           │
└─────────────────────────────────────────────────────┘
```

#### Nuclei Node

```
┌─────────────────────────────────────────────────────┐
│  Node: nuclei                                        │
├─────────────────────────────────────────────────────┤
│  Purpose: Template-based vulnerability scanning      │
│                                                      │
│  Input:   Vec<URL> (targets to scan)                 │
│  Output:  Vec<Finding> (discovered vulnerabilities)  │
│                                                      │
│  Config:                                             │
│    templates: ["cves/", "misconfigurations/"]         │
│    severity_filter: [high, critical]                 │
│    concurrency: 25                                   │
│    rate_limit: 150/s                                 │
│    interactsh_server: ${secrets.INTERACTSH_URL}      │
│                                                      │
│  Transformer:                                        │
│    Input format: JSONL (-jsonl output)               │
│    Mapping:                                          │
│      {"template-id":"cve-2021-44228", ...} →         │
│      Finding {                                       │
│        title: from "info.name",                      │
│        severity: from "info.severity",               │
│        cve: from "info.classification.cve-id",       │
│        cwe: from "info.classification.cwe-id",       │
│        url: from "matched-at",                       │
│        evidence: Evidence {                          │
│          request: from "request",                    │
│          response: from "response",                  │
│          matched_at: from "matcher-name",            │
│        }                                             │
│      }                                               │
│                                                      │
│  Versions supported: v2.x, v3.x                     │
│  Critical difference: v2 uses "template-id",         │
│  v3 uses "template_id" (underscore)                  │
│                                                      │
│  Temp files: target list written to tempfile          │
│  (tempfile crate, auto-deleted on drop via RAII)     │
│                                                      │
│  Stdin: Stdio::null() — nuclei must not block        │
│  waiting for stdin input                             │
└─────────────────────────────────────────────────────┘
```

#### Ffuf Node

```
┌─────────────────────────────────────────────────────┐
│  Node: ffuf                                          │
├─────────────────────────────────────────────────────┤
│  Purpose: Web fuzzing (directory/parameter brute)    │
│                                                      │
│  Input:   Vec<URL> (base URLs to fuzz)               │
│  Output:  Vec<URL> (discovered endpoints)            │
│                                                      │
│  Config:                                             │
│    wordlist: "/usr/share/wordlists/dirbuster/..."    │
│    fuzz_mode: dir | param | header | vhost           │
│    filters:                                          │
│      status_codes: [200, 301, 302, 403]              │
│      size_filter: "!=0"                              │
│    threads: 40                                       │
│    rate_limit: 100/s                                 │
│                                                      │
│  Temp files: wordlist and target list written to     │
│  tempfile (auto-deleted on drop). For crash          │
│  debugging: ~/.achilles/runs/<run-id>/tmp/<node>/    │
│  cleaned on successful workflow completion.          │
│                                                      │
│  Stdin: Stdio::null()                                │
│                                                      │
│  Transformer:                                        │
│    Input format: JSON (-of json output)              │
│    Mapping:                                          │
│      {"url":"...", "status": 200, ...} →             │
│      URL {                                           │
│        full: from "url",                             │
│        response: HTTPResponse {                      │
│          status_code: from "status",                 │
│          content_length: from "length",              │
│        }                                             │
│      }                                               │
└─────────────────────────────────────────────────────┘
```

#### Httpx Node

```
┌─────────────────────────────────────────────────────┐
│  Node: httpx                                         │
├─────────────────────────────────────────────────────┤
│  Purpose: HTTP probing + technology detection        │
│                                                      │
│  Input:   Vec<Host> (hosts to probe for web servers) │
│  Output:  Vec<URL> (alive web endpoints)             │
│                                                      │
│  Config:                                             │
│    ports: [80, 443, 8080, 8443]                      │
│    follow_redirects: true                            │
│    tech_detect: true                                 │
│    status_codes: [200, 301, 302, 401, 403]           │
│    threads: 50                                       │
│                                                      │
│  Stdin: Stdio::null()                                │
│                                                      │
│  SNI/CDN Handling:                                   │
│    When the engine passes pre-resolved IPs           │
│    (§5.7.1 TOCTOU mitigation), TLS handshakes        │
│    against CDN-hosted targets (Cloudflare, Akamai)   │
│    will fail because the SNI header is missing.      │
│    The httpx node MUST pass both the original        │
│    hostname and the resolved IP:                     │
│      httpx -u https://target.com                     │
│             -resolve target.com:<resolved_ip>        │
│    This forces httpx to connect to the resolved IP   │
│    while sending the correct SNI header.             │
│    Nuclei uses the same pattern:                     │
│      nuclei -u https://target.com                    │
│             -resolve target.com:<resolved_ip>        │
│                                                      │
│  Transformer:                                        │
│    Input format: JSONL                               │
│    Mapping:                                          │
│      {"url":"...", "status_code":200, ...} →         │
│      URL {                                           │
│        full: from "url",                             │
│        response: HTTPResponse {                      │
│          status_code, title, content_type,           │
│          content_length, headers                     │
│        },                                            │
│        technologies: from "tech" array               │
│      }                                               │
└─────────────────────────────────────────────────────┘
```

**Subprocess Stdio Policy (all tool nodes):**

Every subprocess spawned by Achilles MUST explicitly close stdin:

```rust
use std::process::Stdio;

Command::new("nmap")
    .arg("-sV").arg(target)
    .stdin(Stdio::null())      // CRITICAL: prevent tool blocking on stdin
    .stdout(Stdio::piped())    // Capture output
    .stderr(Stdio::piped())    // Capture errors
    .spawn()
```

Without `Stdio::null()`, tools that optionally read from stdin (e.g., nuclei, ffuf) will block indefinitely waiting for input that never arrives. The pipe is left open by default, and the tool cannot distinguish "no input yet" from "input coming later." Closing stdin immediately signals EOF.

**Temp File Strategy (all tool nodes):**

Nodes that write input files for tools (target lists, wordlists) use the `tempfile` crate, which auto-deletes files on drop via Rust's RAII:

```rust
use tempfile::NamedTempFile;

let mut targets_file = NamedTempFile::new()?;
for url in &input_urls {
    writeln!(targets_file, "{}", url.full)?;
}
// Pass to tool:
Command::new("nuclei").arg("-l").arg(targets_file.path())
// When targets_file goes out of scope: file is automatically deleted.
```

For files that must survive a crash for debugging, use `~/.achilles/runs/<run-id>/tmp/<node-name>/` instead. These are cleaned on successful workflow completion, but preserved on failure for post-mortem analysis.

#### Sqlmap Node

```
┌─────────────────────────────────────────────────────┐
│  Node: sqlmap                                        │
├─────────────────────────────────────────────────────┤
│  Purpose: SQL injection detection + exploitation     │
│                                                      │
│  Input:   Vec<Finding> (SQLi candidates from nuclei) │
│           OR Vec<URL> (URLs with injectable params)  │
│  Output:  Vec<Finding> (confirmed SQLi findings)     │
│           Vec<Credential> (extracted DB credentials) │
│                                                      │
│  Config:                                             │
│    level: 3 (1-5)                                    │
│    risk: 2 (1-3)                                     │
│    technique: BEUSTQ                                 │
│    threads: 5                                        │
│    dump: false  # requires approval node upstream    │
│    batch: true                                       │
│                                                      │
│  ⚠ REQUIRES APPROVAL NODE UPSTREAM                   │
│    sqlmap performs active exploitation. The engine    │
│    warns if a sqlmap node has no approval node        │
│    between it and the workflow input.                 │
│                                                      │
│  Transformer (HIGH complexity):                      │
│    Input format: stdout text + log files             │
│    Parsing: Regex-based extraction of:               │
│      - "Parameter: id (GET)" → injectable param      │
│      - "Type: boolean-based blind" → technique       │
│      - "Payload: 1 OR 1=1" → evidence               │
│      - Database dump output → Credential objects     │
└─────────────────────────────────────────────────────┘
```

### 6.3 Logic Nodes

Logic nodes perform workflow control — they don't invoke external tools.

#### Conditional Node

```yaml
# Route data based on conditions
- name: severity_filter
  type: conditional
  condition: "input.findings.any(f => f.severity == 'critical')"
  if_true: exploit_branch
  if_false: report_only
```

Evaluates an expression against the ADC objects and routes data flow. The expression language is a safe, sandboxed subset — no arbitrary code execution, only field access, comparisons, and aggregation functions (`any`, `all`, `count`, `filter`).

#### Loop Node

```yaml
# Iterate over a collection, executing a subgraph per item
- name: per_host_scan
  type: loop
  over: "input.hosts"
  max_parallel: 10        # Process 10 hosts in parallel
  body:
    - name: deep_scan
      tool: nmap
      args:
        flags: ["-sV", "-sC", "--script=vuln"]
        targets: "${loop.item.ip_addresses[0]}"
```

Iterates over a collection, executing a sub-workflow for each item. `max_parallel` controls concurrency.

#### Merge Node

```yaml
# Collect outputs from multiple upstream nodes into one stream
- name: collect_results
  type: merge
  inputs: [nmap_scan, nuclei_scan, ffuf_scan]
  strategy: wait_all       # wait_all | first_complete | timeout
  timeout: 60m             # For timeout strategy
```

Aggregates outputs from parallel branches into a single typed stream.

#### Data Transform Node

```yaml
# Transform ADC objects without invoking an external tool
- name: extract_web_hosts
  type: transform
  input_type: Vec<Host>
  output_type: Vec<URL>
  logic: |
    input.hosts
      .filter(h => h.ports.any(p => [80,443,8080,8443].contains(p.number)))
      .flat_map(h => h.web_urls())
```

Converts between ADC types using the same safe expression language as conditional nodes.

#### Split Node

```yaml
# Split a data stream based on a property
- name: split_by_severity
  type: split
  input_type: Vec<Finding>
  split_on: "severity"
  outputs:
    critical: critical_branch
    high: high_branch
    default: low_priority_branch
```

Routes objects to different branches based on a field value. Similar to a switch statement.

### 6.3.1 Expression Language Implementation

The conditional, transform, and split nodes all reference expressions like `input.findings.any(f => f.severity == "critical")`. This section specifies exactly what technology implements these expressions, because the choice determines security surface, complexity, and contributor onboarding.

**Three options evaluated honestly:**

| Option | Implementation Effort | Security Surface | Performance | No Arbitrary Code Execution? |
|---|---|---|---|---|
| **Custom DSL (nom parser)** | ~3,000-5,000 LOC. Hand-rolled lexer, parser, type checker, evaluator. Requires designing a grammar, writing error messages, building a test suite from scratch. | Minimal — you control every function. Attack surface = your parser bugs. | Excellent — compiled Rust evaluating a minimal AST. | ✅ By construction — only features you implement exist. |
| **Rhai embedded scripting** | ~500 LOC integration. Rhai provides the engine, AST, type system, and sandboxing. You register ADC types and allowed functions. | Small — Rhai is designed for embedding with sandboxing as a first-class feature. Attack surface = Rhai's sandbox correctness (well-tested, used in production by multiple projects). | Good — interpreted, but fast enough for expression evaluation on collections of <100K objects. Not fast enough for per-byte data processing (irrelevant for Achilles). | ✅ With configuration — disable `eval`, file I/O, system access, module imports. Default Rhai has no I/O capability. |
| **CEL (Common Expression Language)** | ~800 LOC integration using the `cel-interpreter` Rust crate. CEL is a Google-designed expression language with formal specification. | Small — CEL is spec-defined with a formal security model. No side effects by design. | Excellent — compiled to bytecode. Designed for policy evaluation at Google-scale (used in Firebase Security Rules, Envoy proxy). | ✅ By design — CEL has no side effects, no assignments, no loops. It is a pure expression language. |

**Analysis:**

A custom DSL built with `nom` is the most secure option in theory (you control everything) but the worst option in practice. A hand-rolled expression language takes months to reach production quality — error messages alone take weeks. For a solo developer, this is a project-killing yak shave.

CEL is technically ideal — it is literally designed for safe evaluation of expressions over structured data with no side effects. However, the Rust CEL ecosystem is immature. The `cel-interpreter` crate has limited adoption, incomplete spec coverage, and no established maintenance trajectory. Using it means betting on a crate that may stall.

**Recommendation: Rhai.**

Rhai is the correct choice because it offers the best ratio of integration effort to safety guarantees. It is purpose-built for embedding in Rust applications, its sandbox model is mature and well-tested, and the integration requires ~500 lines — not ~5,000. The security constraint (no arbitrary code execution) is met by disabling Rhai's optional features at engine initialization, not by trusting the expression author.

**Expression Language Grammar (Rhai subset exposed to Achilles):**

```
EXPRESSION LANGUAGE — ACHILLES SAFE SUBSET

Field access:
  input.findings              Access a typed ADC collection
  f.severity                  Access a field on an ADC object
  h.ports[0].number           Indexed access on arrays
  h.hostnames.len()           Built-in collection methods

Comparison operators:
  ==   !=   <   >   <=   >=

Logical operators:
  &&   ||   !

String operators:
  .contains("substr")         Substring match
  .starts_with("prefix")      Prefix match
  .ends_with("suffix")        Suffix match
  .to_lower()                 Case normalization

Aggregation functions (on collections):
  .any(|item| predicate)      True if any item matches
  .all(|item| predicate)      True if all items match
  .filter(|item| predicate)   Return matching items
  .map(|item| transform)      Transform each item
  .count()                    Number of items
  .flat_map(|item| expr)      Flatten nested collections
  .sort_by(|a, b| compare)    Sort collection

Literals:
  "string"                    String literals
  42, 3.14                    Numeric literals
  true, false                 Boolean literals
  [80, 443, 8080]             Array literals

EXPLICITLY FORBIDDEN:
  ✗ Variable assignment (let x = ...)
  ✗ eval() or dynamic code execution
  ✗ File I/O (read, write, open)
  ✗ Network access (HTTP, DNS)
  ✗ System calls (exec, spawn)
  ✗ Module imports (import, use)
  ✗ Infinite loops (while, loop)
  ✗ Mutable state across expressions
  ✗ Access to Rust std:: functions
```

The Rhai engine is configured at Achilles startup with these restrictions:

```rust
let mut engine = Engine::new();

// Disable everything dangerous
engine.set_max_expr_depths(64, 32);    // Prevent stack overflow
engine.set_max_string_size(10_000);     // Prevent memory exhaustion
engine.set_max_array_size(100_000);     // Prevent OOM on large collections
engine.set_max_operations(1_000_000);   // Prevent infinite computation
engine.disable_symbol("eval");          // No dynamic evaluation

// Register ONLY the ADC types and allowed functions
engine.register_type_with_name::<Host>("Host");
engine.register_type_with_name::<Finding>("Finding");
engine.register_type_with_name::<Port>("Port");
// ... register all ADC types

// Register collection functions
engine.register_fn("any", adc_any);
engine.register_fn("all", adc_all);
engine.register_fn("filter", adc_filter);
engine.register_fn("count", adc_count);
```

The engine sees only what Achilles explicitly registers. If an ADC type does not have a registered accessor, Rhai cannot read it. This is the **principle of least privilege applied to expression evaluation**: the expression can see findings, hosts, and ports because the engine decided it should. It cannot see secrets, file paths, or system state because the engine never registered them.

### 6.4 Custom Script Node

Custom script nodes allow users to write arbitrary transformation logic. They are sandboxed in WebAssembly (see I-02).

```yaml
- name: custom_enrichment
  type: custom_script
  language: python          # Compiled to WASM via Pyodide or natively
  capabilities:
    network:
      allow: ["api.shodan.io"]
    time_limit: 60s
    memory_limit: 256MB
  script: |
    from achilles_sdk import input, output, secrets

    hosts = input.get_all("Host")
    shodan_key = secrets.get("SHODAN_KEY")

    for host in hosts:
        # Enrich each host with Shodan data
        enriched = shodan_lookup(host.ip_addresses[0], shodan_key)
        host.metadata["shodan"] = enriched
        output.emit(host)
```

**SDK API available to custom scripts:**

| Function | Description |
|---|---|
| `input.get_all(type)` | Get all ADC objects of a type from upstream nodes |
| `input.get(type, from_node)` | Get ADC objects from a specific upstream node |
| `output.emit(object)` | Emit an ADC object to downstream nodes |
| `output.emit_finding(...)` | Convenience: create and emit a Finding |
| `secrets.get(key)` | Read a secret from the encrypted vault |
| `log.info(msg)` | Log a message (visible in workflow output) |
| `log.warn(msg)` | Log a warning |
| `http.get(url)` | Make HTTP request (only to allowed hosts) |
| `http.post(url, body)` | Make HTTP POST (only to allowed hosts) |

### 6.5 Community Node Contribution

Community members can contribute new tool nodes to the Achilles Node Library:

**Contribution process:**
1. Fork the Achilles repository
2. Create a new node implementation in `nodes/community/<tool_name>/`
3. Implement the `Node` trait
4. Write the transformer with test cases (minimum 5 real-world tool outputs)
5. Document: input schema, output schema, config options, supported versions
6. Submit PR → automated CI runs: schema validation, transformer tests, sandbox compliance check
7. Core team reviews: security audit of the transformer (no arbitrary code execution), correctness check against real tool output
8. Merge → available in next Achilles release

**Node package format:**
```
nodes/community/masscan/
├── mod.rs              # Node trait implementation
├── transformer.rs      # Raw output → ADC mapping
├── config.rs           # Configuration schema
├── tests/
│   ├── masscan_v1.3_output.json
│   ├── masscan_v1.3_expected_adc.json
│   ├── test_transformer.rs
│   └── test_scope_filtering.rs
└── README.md           # Usage documentation
```

---

## 7. Secrets & Security Architecture

### 7.1 Formal Threat Model (STRIDE)

A security architecture is only as valid as its threat model. This section defines Achilles's threat actors, applies the STRIDE framework to enumerate threats, and traces every security decision in §7.2–7.4 back to a specific threat. The goal: **every defense exists because of a stated threat. No defense is arbitrary.**

#### 7.1.1 Threat Actors

| Actor | Description | Capability | Motivation |
|---|---|---|---|
| **TA-1: Malicious Workflow Author** | Creates a community workflow with hidden malicious nodes | Can craft arbitrary YAML, embed custom scripts, define misleading node names | Exfiltrate secrets, pivot to operator's network, use operator as proxy for attacks |
| **TA-2: Compromised Upstream Tool** | A supply chain attack where a tool binary (nmap, nuclei) is replaced with a trojanized version | Full code execution within the tool's process, can produce crafted output | Inject malicious results, exfiltrate data via tool's stdout, backdoor the operator's system |
| **TA-3: Careless Operator** | Legitimate user who makes mistakes — wrong scope, runs untested workflows in production, ignores warnings | Full access to Achilles, runs workflows with valid credentials | None (unintentional) — but the damage from scanning out of scope or leaking credentials is identical to an intentional attack |
| **TA-4: Tool Output Injector** | An attacker who controls a target and crafts responses to inject malicious data into Achilles's pipeline | Can return crafted HTTP responses, DNS records, or service banners | Inject false findings, cause Achilles to scan attacker-controlled targets (scope escape via redirect), corrupt the data pipeline |

#### 7.1.2 STRIDE Analysis

```
┌──────────────────────────────────────────────────────────────────┐
│                    STRIDE THREAT MODEL                          │
├──────────────┬──────────────────────────────────────────────────┤
│  Category    │  Threats → Defenses                              │
├──────────────┼──────────────────────────────────────────────────┤
│              │                                                  │
│  SPOOFING    │  TA-1 authors workflow as "@trusted_researcher"  │
│              │  → Workflow hash + author verification (§7.3)    │
│              │                                                  │
│              │  TA-2 replaces nmap binary with trojan            │
│              │  → Tool binary hash verification (I-02)          │
│              │                                                  │
│  TAMPERING   │  TA-1 modifies a shared workflow after review    │
│              │  → Workflow hash comparison on re-run (§7.3)     │
│              │                                                  │
│              │  TA-3 edits audit log to hide scope violation     │
│              │  → Cryptographic hash chain (§7.4, I-11)         │
│              │                                                  │
│              │  TA-4 injects crafted nmap XML with malicious     │
│              │  hostnames containing shell metacharacters        │
│              │  → ADC schema validation strips non-conforming   │
│              │    data before downstream processing (§4.6)      │
│              │                                                  │
│  REPUDIATION │  TA-3 claims "I never ran that scan"              │
│              │  → Tamper-evident audit log with hash chain       │
│              │    proves every action (§7.4)                    │
│              │                                                  │
│              │  TA-3 claims "I didn't approve exploitation"      │
│              │  → Approval node logs operator identity +         │
│              │    timestamp + approval scope (§5.6)             │
│              │                                                  │
│  INFO DISC.  │  TA-1 custom script reads secrets and exfils     │
│              │  → WASM sandbox + network allowlist (I-02, I-08) │
│              │                                                  │
│              │  Secrets visible in `ps aux` command list         │
│              │  → Env variable injection, not CLI args (I-08)   │
│              │                                                  │
│              │  Secrets in log files or SQLite state             │
│              │  → Log scrubbing + state store encryption (§7.4) │
│              │                                                  │
│  DENIAL SVC  │  TA-1 crafts workflow with infinite loop          │
│              │  → DAG cycle detection rejects at validation      │
│              │    (§5.1)                                        │
│              │                                                  │
│              │  TA-1 node consumes all CPU/memory (fork bomb)    │
│              │  → cgroups v2 resource limits per node (I-12)    │
│              │                                                  │
│              │  TA-4 returns enormous response to exhaust memory │
│              │  → Per-node output size limits (I-12)            │
│              │                                                  │
│  ELEV PRIV   │  TA-1 custom script escapes WASM sandbox         │
│              │  → Wasmtime capability model: no FS/net unless   │
│              │    explicitly granted (I-02)                     │
│              │                                                  │
│              │  TA-4 redirects tool to out-of-scope target       │
│              │  → Post-node scope enforcement catches output    │
│              │    targets not in scope (§5.7, I-04)             │
│              │                                                  │
└──────────────┴──────────────────────────────────────────────────┘
```

#### 7.1.3 Threat-to-Defense Traceability Matrix

Every security control in Achilles exists because of a specific threat. No defense is arbitrary.

| Defense | Section | Protects Against | Threat Actor |
|---|---|---|---|
| Workflow schema validation | §7.3, I-10 | Malformed/malicious workflow structure | TA-1 |
| Static analysis of workflow | §7.3, I-10 | Shell injection in tool arguments, YAML bombs | TA-1 |
| Capability declaration + consent | §7.3, I-10 | Hidden network access, filesystem access in custom scripts | TA-1 |
| WASM sandbox (Wasmtime) | §6.4, I-02 | Arbitrary code execution, filesystem access, uncontrolled network access | TA-1 |
| Network allowlist for custom scripts | I-02, I-08 | Secret exfiltration via HTTP to attacker server | TA-1 |
| Three-point scope enforcement | §5.7, I-04 | Scanning out-of-scope targets (intentional or via redirect) | TA-3, TA-4 |
| DNS resolution cross-check | §5.7, I-04 | Shared hosting scope escape (domain resolves to shared IP) | TA-4 |
| Secrets vault (AES-256-GCM) | I-08 | Credential theft from disk | External attacker |
| Environment variable injection | I-08 | Credentials visible in `ps aux` | Local attacker |
| Log scrubbing | §7.4, I-08 | Credentials appearing in log files | TA-3 (accidental), TA-1 (intentional) |
| Audit hash chain | §7.4, I-11 | Tampering with audit trail to hide actions | TA-3 |
| cgroups v2 resource limits | I-12 | Fork bombs, memory exhaustion, CPU starvation | TA-1, TA-4 |
| DAG cycle detection | §5.1 | Infinite loops from circular dependencies | TA-1 |
| Tool binary hash verification | I-02 | Supply chain attacks on upstream tool binaries | TA-2 |
| ADC schema validation on all data | §4.6 | Injection of malformed data from crafted tool responses | TA-4 |
| Approval node with default-deny | §5.6, I-07 | Automated exploitation without human review | TA-1, TA-3 |

### 7.2 Defense-in-Depth Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    SECURITY ARCHITECTURE LAYERS                   │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Layer 1: Workflow Validation                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Schema validation → Static analysis → Capability review  │  │
│  │  → User consent                                           │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Layer 2: Scope Enforcement                                      │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Pre-workflow → Pre-node → Post-node (continuous)         │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Layer 3: Execution Sandbox                                      │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Tool nodes: cgroups v2 (CPU, memory, network limits)     │  │
│  │  Custom scripts: WASM sandbox (Wasmtime)                  │  │
│  │  All nodes: process isolation, no shared filesystem       │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Layer 4: Secrets Protection                                     │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Encrypted vault → Runtime injection → Memory zeroing     │  │
│  │  → Log scrubbing → Audit trail (usage, not values)        │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  Layer 5: Audit & Accountability                                 │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Hash chain log → Every action recorded → Tamper-evident  │  │
│  │  → Legal defensibility                                    │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 7.3 Workflow File Validation Deep-Dive

When an operator runs a workflow for the first time (or runs a modified workflow), Achilles performs a **security review** that produces a human-readable summary:

```
$ achilles run community/bug_bounty_recon.yaml

╔══════════════════════════════════════════════════════════════╗
║                   WORKFLOW SECURITY REVIEW                  ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  Workflow: Bug Bounty Recon Pipeline                         ║
║  Author:   @security_researcher                              ║
║  Version:  1.2.0                                             ║
║  Hash:     sha256:a1b2c3d4e5f6...                            ║
║                                                              ║
║  SCOPE:                                                      ║
║    Targets: *.target.com, 10.0.0.0/24                        ║
║    Excludes: mail.target.com, vpn.target.com                 ║
║                                                              ║
║  TOOLS USED:                                                 ║
║    subfinder (passive recon)                                 ║
║    httpx (HTTP probing)                                      ║
║    nmap (port scanning — ACTIVE)                             ║
║    nuclei (vuln scanning — ACTIVE)                           ║
║    ffuf (directory fuzzing — ACTIVE)                         ║
║                                                              ║
║  CUSTOM SCRIPTS: 1                                           ║
║    "shodan_enrichment" — network access to: api.shodan.io    ║
║                                                              ║
║  SECRETS REQUIRED:                                           ║
║    SHODAN_KEY, CENSYS_KEY                                    ║
║                                                              ║
║  APPROVAL GATES: 1                                           ║
║    Before: nuclei → sqlmap (exploitation step)               ║
║                                                              ║
║  ⚠ WARNINGS:                                                ║
║    - nmap uses -sV flag (active service detection)           ║
║    - ffuf uses 40 threads (high request rate)                ║
║                                                              ║
║  Proceed? [y/N/inspect]                                      ║
╚══════════════════════════════════════════════════════════════╝
```

### 7.4 Audit Logging Implementation

Every action that Achilles performs is logged to `~/.achilles/audit/<run-id>.log`:

```rust
pub struct AuditLogger {
    /// File handle to the append-only log
    log_file: File,
    /// Previous entry's hash (for chain integrity)
    prev_hash: String,
    /// Sequence counter
    sequence: u64,
    /// Known secret values (for scrubbing)
    secret_values: HashSet<String>,
}

impl AuditLogger {
    pub fn log(&mut self, entry: AuditEntry) -> Result<()> {
        // 1. Scrub any secret values from the entry
        let scrubbed = self.scrub_secrets(entry);

        // 2. Compute entry hash (chained)
        let serialized = serde_json::to_string(&scrubbed)?;
        let entry_hash = sha256(format!("{}{}", self.prev_hash, serialized));

        // 3. Write to log
        let record = AuditRecord {
            sequence: self.sequence,
            entry: scrubbed,
            prev_hash: self.prev_hash.clone(),
            entry_hash: entry_hash.clone(),
        };
        writeln!(self.log_file, "{}", serde_json::to_string(&record)?)?;
        self.log_file.sync_all()?; // Flush to disk immediately

        // 4. Update state
        self.prev_hash = entry_hash;
        self.sequence += 1;
        Ok(())
    }

    /// Scrub secrets using Aho-Corasick multi-pattern matching.
    /// One pass over the output, all secrets matched simultaneously, O(n) in output length.
    /// Naive string::replace with N secrets is O(n*N) — unacceptable for 100MB+ nmap outputs.
    fn scrub_secrets(&self, entry: AuditEntry) -> AuditEntry {
        if self.secret_scrubber.is_none() {
            return entry; // No secrets registered
        }
        let scrubber = self.secret_scrubber.as_ref().unwrap();
        let serialized = serde_json::to_string(&entry).unwrap();
        let scrubbed = scrubber.replace_all(&serialized);
        serde_json::from_str(&scrubbed).unwrap()
    }
}

/// High-performance multi-pattern secret scrubber using Aho-Corasick.
/// Replaces all known secret values in a single O(n) pass.
struct SecretScrubber {
    automaton: aho_corasick::AhoCorasick,
    replacements: Vec<String>,
}

impl SecretScrubber {
    fn new(secrets: &HashMap<String, String>) -> Self {
        let patterns: Vec<&str> = secrets.keys().map(|s| s.as_str()).collect();
        let replacements: Vec<String> = secrets.values()
            .map(|name| format!("[REDACTED:{}]", name))
            .collect();
        Self {
            automaton: aho_corasick::AhoCorasick::new(&patterns).unwrap(),
            replacements,
        }
    }

    fn replace_all(&self, text: &str) -> String {
        self.automaton.replace_all(text, &self.replacements)
    }
}
```

### 7.5 Runtime Credential Scrubbing

The secrets management system (§7.4, I-08) handles credentials the operator stores in the vault — API keys, tokens, passwords used to authenticate to services. But there is a second class of sensitive data that the vault does not know about: **credentials extracted from targets at runtime.**

When sqlmap extracts a plaintext password from a vulnerable database, that password becomes a `Credential` ADC object:

```rust
Credential {
    id: "cred-001",
    username: "admin",
    password_hash: "$2b$12$LJ3m...",    // Could be plaintext if DB stores plaintext
    plaintext_password: Some("P@ssw0rd123"),  // sqlmap sometimes recovers this
    source: SourceInfo { tool: "sqlmap", node_id: "sqli_exploit" },
    host_ref: "host-042",
}
```

This `Credential` object flows through the DAG — into merge nodes, potentially into custom scripts, and into the report node. Without explicit handling, plaintext passwords will appear in:
- The SQLite state store (checkpointed as `output_json`)
- The audit log (if the credential appears in a log entry)
- The final report (if the report template naively lists all findings with evidence)
- Custom script input (if a script node receives `Vec<Credential>`)

**Engine-Level Credential Tracking:**

The engine maintains a runtime registry of all `Credential` objects produced during a workflow:

```rust
struct CredentialRegistry {
    /// All plaintext passwords extracted during this run
    sensitive_values: HashSet<String>,
    /// Mapping from credential ID to redacted display form
    redacted_forms: HashMap<String, String>,
}

impl CredentialRegistry {
    /// Minimum password length for global text scrubbing.
    /// Passwords shorter than this are NOT added to the global AuditLogger
    /// scrubber because they would corrupt unrelated data.
    /// Example: password "admin" would match the string "admin" in
    /// node names, usernames, log messages, and JSON keys — destroying
    /// the audit log's integrity.
    const MIN_GLOBAL_SCRUB_LENGTH: usize = 8;

    fn register(&mut self, cred: &Credential) {
        if let Some(ref plaintext) = cred.plaintext_password {
            let redacted = format!(
                "[EXTRACTED_CREDENTIAL:{}@{}]",
                cred.username, cred.host_ref
            );

            if plaintext.len() >= Self::MIN_GLOBAL_SCRUB_LENGTH {
                // Safe for global text scrubbing — long enough to be unique
                self.sensitive_values.insert(plaintext.clone());
            } else {
                // SHORT PASSWORD: do NOT add to global scrubber.
                // Instead, scrub only within Credential-typed fields:
                // the engine strips plaintext_password from Credential
                // objects before serialization to logs/state, but never
                // does a blind find-replace across all text output.
                // This prevents "123" from corrupting port numbers,
                // status codes, and IP address octets in the audit log.
            }

            self.redacted_forms.insert(cred.id.clone(), redacted);
        }
    }
}
```

When a node produces a `Credential` with a non-`None` `plaintext_password`, the engine immediately registers the plaintext value with both the `CredentialRegistry` and the `AuditLogger`'s `secret_values` set. From that point forward, the log scrubber treats the extracted password identically to a vault secret — it is replaced with `[REDACTED:extracted_cred:cred-001]` in all output.

**CredentialRegistry Lifecycle Across Tokio Threads:**

The DAG scheduler spawns node executions as Tokio tasks across multiple threads. The `CredentialRegistry` must be shared safely without introducing locking contention that would serialize the parallel executor.

The registry is wrapped in `Arc<RwLock<CredentialRegistry>>` — not `Arc<Mutex>`. The access pattern is **read-heavy, write-rare**: most nodes read the registry (to scrub their logs) while only nodes that produce `Credential` objects (sqlmap, custom scripts) write to it. `RwLock` allows unlimited concurrent readers with exclusive writer access:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ExecutionContext {
    pub scope_engine: Arc<ScopeEngine>,
    pub audit_logger: Arc<RwLock<AuditLogger>>,
    pub credential_registry: Arc<RwLock<CredentialRegistry>>,
    // ...
}

// Node execution — WRITING (rare: only sqlmap, credential-producing nodes)
async fn post_node_processing(ctx: &ExecutionContext, output: &NodeOutput) {
    for obj in &output.objects {
        if let ADCObject::Credential(cred) = obj {
            // Acquire write lock — blocks readers momentarily
            let mut registry = ctx.credential_registry.write().await;
            registry.register(cred);

            // Also register with the audit logger's scrubber
            if let Some(ref plaintext) = cred.plaintext_password {
                let mut logger = ctx.audit_logger.write().await;
                logger.secret_values.insert(plaintext.clone());
            }
            // Write lock released here — readers resume
        }
    }
}

// Log scrubbing — READING (frequent: every node, every log entry)
async fn scrub_output(ctx: &ExecutionContext, text: &str) -> String {
    // Acquire read lock — does NOT block other readers
    let registry = ctx.credential_registry.read().await;
    let mut scrubbed = text.to_string();
    for (value, redacted) in registry.sensitive_values.iter()
        .zip(registry.redacted_forms.values()) {
        scrubbed = scrubbed.replace(value, redacted);
    }
    scrubbed
}
```

The `tokio::sync::RwLock` (not `std::sync::RwLock`) is used because the lock is held across `.await` points. `std::sync::RwLock` would block the Tokio thread; `tokio::sync::RwLock` yields to the runtime while waiting for the lock.

**Contention analysis:** In a typical workflow, 0-2 nodes produce credentials (only sqlmap and custom scripts). All other nodes only read the registry for log scrubbing. With `RwLock`, the parallel execution of 10 nodes proceeds without any lock contention unless a credential-producing node is actively writing — a window of microseconds per credential. This is negligible compared to the seconds-to-minutes each node spends executing external tools.

**SQLite State Store:**

`Credential` objects are written to the SQLite state store with the `plaintext_password` field encrypted separately using the same AES-256-GCM key as the secrets vault. The `output_json` column contains the credential with `plaintext_password: "[ENCRYPTED]"`. The actual value is stored in a separate `encrypted_credentials` table:

```sql
CREATE TABLE encrypted_credentials (
    run_id TEXT REFERENCES workflow_runs(run_id),
    credential_id TEXT NOT NULL,
    encrypted_password BLOB NOT NULL,    -- AES-256-GCM encrypted
    nonce BLOB NOT NULL,
    PRIMARY KEY (run_id, credential_id)
);
```

This ensures that even if an attacker obtains the SQLite database file, extracted credentials are not readable without the vault key.

**Report Node Handling:**

The report node applies a credential display policy:

| Report Format | Credential Display | Rationale |
|---|---|---|
| **JSON (machine-readable)** | `plaintext_password` field set to `"[REDACTED]"` with a `credential_ref` pointing to the encrypted store | Downstream tools that need the credential can request it via `achilles creds export` with authentication |
| **Markdown (client report)** | Shows `username`, `host`, `source` — never the password. Includes a note: "Extracted credentials are stored securely and available upon request." | Client reports should not contain plaintext passwords — they are often emailed, stored in shared drives, or printed |
| **HTML (styled report)** | Same as Markdown | Same rationale |

A `--include-credentials` flag exists for the report node but requires an additional approval node to be present upstream. This prevents accidental inclusion of plaintext credentials in reports.

**Custom Script Node Policy:**

Custom script nodes (WASM sandbox) have a capability declaration for credential access with two distinct levels:

| Capability | What the script sees | When to use | Security review flag |
|---|---|---|---|
| `credentials: read_metadata` **(default)** | `Credential { id, username, host_ref, source, password_hash: None, plaintext_password: None }` — password fields are stripped before the object reaches the sandbox | Enrichment scripts that need to know which hosts had credentials extracted, but don't need the actual passwords | None — transparent to the operator |
| `credentials: read_full` | Full `Credential` object including `plaintext_password` if present | Scripts that perform credential analysis (e.g., password strength checking, credential reuse detection) | ⚠ **HIGH RISK** — flagged in §7.3 security review summary |

```yaml
- name: credential_enrichment
  type: custom_script
  capabilities:
    credentials: read_metadata    # Default — safe
```

```yaml
- name: password_strength_check
  type: custom_script
  capabilities:
    credentials: read_full        # Requires explicit approval in security review
```

**Implementation — field stripping at the engine level:**

The credential capability is enforced by the engine before the WASM module receives any data — not by the WASM module itself. The engine constructs a filtered view of the `Credential` object:

```rust
fn filter_credential_for_capability(
    cred: &Credential,
    capability: CredentialCapability,
) -> Credential {
    match capability {
        CredentialCapability::ReadMetadata => Credential {
            id: cred.id.clone(),
            username: cred.username.clone(),
            host_ref: cred.host_ref.clone(),
            source: cred.source.clone(),
            // Strip sensitive fields
            password_hash: None,
            plaintext_password: None,
        },
        CredentialCapability::ReadFull => cred.clone(),
    }
}
```

This means a WASM module cannot bypass the filter — it receives only the fields the engine decides to provide. Even if the WASM code attempts to access `plaintext_password`, the field is `None` at the Rust level before serialization into the sandbox.

**Workflow validation rule:** If a workflow YAML declares `credentials: read_full` on any custom script node, the engine requires an approval node upstream of that script node. This is the same gate pattern used for sqlmap (§6.2) — high-risk capabilities require human confirmation before execution.

---

## 8. CLI Design & Workflow Portability

### 8.1 Command Structure

The Achilles CLI follows the `verb-noun` pattern used by modern CLI tools (kubectl, docker, gh):

```
achilles <command> [subcommand] [flags] [arguments]

COMMANDS:

  Workflow Execution:
    run <workflow.yaml>              Execute a workflow
    resume <run-id>                  Resume a failed/paused workflow
    approve <run-id> <approval-id>   Approve a pending approval node
    abort <run-id>                   Abort a running workflow
    status <run-id>                  Show status of a workflow run

  Workflow Management:
    new [template]                   Create a new workflow from template
    validate <workflow.yaml>         Validate a workflow without executing
    inspect <workflow.yaml>          Show security review + data flow diagram
    list                             List available workflows in current directory
    graph <workflow.yaml>            Generate ASCII or DOT graph of the DAG

  Node Management:
    nodes list                       List all available nodes (built-in + community)
    nodes info <node-type>           Show node's input/output schema + config options
    nodes install <package>          Install a community node package

  Secrets:
    secrets set <key> <value>        Add a secret to the vault
    secrets list                     List secret keys (not values)
    secrets delete <key>             Remove a secret
    secrets export                   Export encrypted vault for backup

  Template Library:
    templates list                   List available workflow templates
    templates download <name>        Download a template to current directory
    templates share <workflow.yaml>  Share a workflow to the community registry

  Configuration:
    config set <key> <value>         Set a configuration option
    config get <key>                 Get a configuration option
    config init                      Initialize Achilles configuration

GLOBAL FLAGS:
    -v, --verbose                    Verbose output
    -q, --quiet                      Quiet mode (errors only)
    --no-color                       Disable colored output
    --json                           Output in JSON format (for scripting)
    --config <path>                  Custom config file path
    --log-level <level>              Log level: debug, info, warn, error
```

### 8.2 Workflow File Format: YAML (with justification)

**Decision: YAML over JSON.**

| Criterion | YAML | JSON |
|---|---|---|
| **Human readability** | ✅ Superior — no braces, no quotes on keys | ❌ Verbose — every key quoted, nested braces |
| **Comments** | ✅ Native `#` comments | ❌ No comment support |
| **Multi-line strings** | ✅ Native `\|` and `>` syntax (for custom scripts) | ❌ Escaped newlines only |
| **Tool ecosystem** | ✅ ansible, docker-compose, k8s, GitHub Actions all use YAML | ✅ Universal parsing support |
| **Security risks** | ⚠ YAML bombs via anchors (mitigated by limiting anchor depth) | ✅ No bombs |
| **Type coercion** | ⚠ "Norway problem" (`NO` parsed as boolean) — mitigated by strict schema validation | ✅ Explicit types |
| **Familiarity for target audience** | ✅ Security professionals know YAML from ansible, docker, nuclei templates | ✅ Also familiar |

YAML wins on readability and comments — both critical for workflows that operators need to review for security. The security risks (YAML bombs, type coercion) are fully mitigated by the schema validation stage.

**Canonical workflow file structure:**

```yaml
# Achilles Workflow File
# Schema version and metadata
achilles: "1.0"
name: "Bug Bounty Recon Pipeline"
description: "Full-scope recon from domain to vulnerability findings"
author: "operator@example.com"
version: "1.2.0"
tags: [recon, bug-bounty, web]

# Scope enforcement
scope:
  domains:
    include: ["*.target.com", "target.io"]
    exclude: ["mail.target.com", "vpn.target.com"]
  ips:
    include: ["10.0.0.0/24"]
  ports:
    exclude: [445, 135, 139]  # No SMB testing

# Global resource limits
resources:
  max_duration: 4h
  max_concurrent_nodes: 5

# Workflow DAG
nodes:
  - name: discover_subdomains
    tool: subfinder
    config:
      sources: [crtsh, hackertarget, dnsdumpster]
      threads: 10
    output: hosts

  - name: probe_alive
    tool: httpx
    depends_on: [discover_subdomains]
    config:
      ports: [80, 443, 8080, 8443]
      tech_detect: true
    input:
      hosts: "${discover_subdomains.output.hosts}"
    output: urls

  - name: port_scan
    tool: nmap
    depends_on: [discover_subdomains]
    config:
      scan_type: syn
      ports: "top-1000"
      flags: ["-sV", "-sC"]
    input:
      hosts: "${discover_subdomains.output.hosts}"
    output: enriched_hosts
    resources:
      time_limit: 60m
      max_concurrent: 10

  - name: vuln_scan
    tool: nuclei
    depends_on: [probe_alive]
    config:
      templates: ["cves/", "misconfigurations/", "exposures/"]
      severity_filter: [medium, high, critical]
      concurrency: 25
      rate_limit: 150
    input:
      urls: "${probe_alive.output.urls}"
    output: findings
    on_error:
      strategy: retry
      max_retries: 2

  - name: dir_fuzz
    tool: ffuf
    depends_on: [probe_alive]
    config:
      wordlist: "common.txt"
      threads: 40
      filters:
        status_codes: [200, 301, 302, 403]
    input:
      urls: "${probe_alive.output.urls}"
    output: discovered_urls

  - name: approve_exploitation
    type: approval
    depends_on: [vuln_scan]
    config:
      message: "Review {{count(findings)}} findings before exploitation"
      channels: [slack, cli]
      timeout: 2h
    input:
      findings: "${vuln_scan.output.findings}"
    condition: "input.findings.any(f => f.severity == 'critical')"

  - name: sql_injection
    tool: sqlmap
    depends_on: [approve_exploitation]
    config:
      level: 3
      risk: 2
      batch: true
    input:
      findings: "${approve_exploitation.output.findings}"
      filter: "finding_type == 'Vulnerability' && cwe.contains('CWE-89')"
    output: confirmed_sqli

  - name: merge_results
    type: merge
    depends_on: [port_scan, vuln_scan, dir_fuzz, sql_injection]
    strategy: wait_all

  - name: generate_report
    type: report
    depends_on: [merge_results]
    config:
      format: [markdown, json, pdf]
      output_dir: "./reports/"
      include_evidence: true
```

### 8.3 Workflow Portability

Portability across environments is achieved through three mechanisms:

**1. Tool Abstraction:**
Workflows reference tool names (`nmap`), not paths (`/usr/bin/nmap`). The Tool Resolver finds the correct binary at runtime.

**2. Version Constraints:**
```yaml
tool: nmap
version: ">=7.80 <8.0"  # Semver range
```
If the installed version doesn't satisfy the constraint, Achilles emits a clear error suggesting how to install the correct version.

**3. Wordlist Resolution:**
Wordlists are referenced by name, not path:
```yaml
wordlist: "common.txt"  # Resolved from:
                        # 1. ~/.achilles/wordlists/common.txt
                        # 2. /usr/share/wordlists/common.txt (Kali default)
                        # 3. ./wordlists/common.txt (project-local)
```

### 8.4 The Hacker Template Library

Pre-built, battle-tested workflow templates for common engagement types:

| Template | Description | Nodes | Estimated Runtime |
|---|---|---|---|
| **Bug Bounty Recon** | Full recon from single domain to findings | subfinder → httpx → nmap → nuclei → ffuf → report | 1-4 hours |
| **AD Enumeration** | Active Directory discovery and enumeration | nmap → enum4linux → ldapsearch → bloodhound-ingestor → report | 30-60 min |
| **Web App Assessment** | OWASP Top 10 coverage | httpx → nuclei → nikto → sqlmap (with approval) → xsstrike → report | 2-6 hours |
| **API Security Testing** | REST/GraphQL API assessment | openapi_parser → nuclei_api → ffuf_params → auth_test → report | 1-3 hours |
| **Network Pentest** | Internal network assessment | nmap_discovery → nmap_full → vuln_scan → smb_enum → report | 4-12 hours |
| **Cloud Asset Discovery** | Multi-cloud asset enumeration | subfinder → httpx → cloud_enum → s3_scanner → report | 1-3 hours |
| **Red Team Recon** | Deep OSINT + infrastructure mapping | amass → httpx → nmap → nuclei → shodan → censys → report | 6-24 hours |

Each template includes:
- Pre-configured scope placeholders for the operator to fill in
- Approval nodes at appropriate risk boundaries
- Sensible resource limits and rate limiting
- Documentation explaining what each node does and why

---

## 9. Technology Stack & Infrastructure

### 9.1 Language Choice: Rust over Go (steelmanned)

**Decision: Rust. But Go deserves its strongest argument first.**

Python is eliminated immediately. Achilles is not a security tool — it is an **engine that orchestrates security tools.** The engine must parse XML documents with millions of elements (nmap output), transform data at high throughput, manage concurrent processes, and enforce sandboxing. Python's GIL, dynamic typing, and runtime overhead disqualify it for this layer. The security tools themselves can be Python; the engine that runs them cannot.

The real debate is **Go vs Rust.** Here is Go's strongest case, stated honestly:

**The case FOR Go:**

Go is not just a plausible choice — it is the **dominant language** in the security tool ecosystem that Achilles integrates with. subfinder, httpx, nuclei, ffuf, katana, amass — all Go. This matters:

| Go's Advantage | Why It Matters for Achilles |
|---|---|
| **Single static binary** | Same as Rust. `go build` → one file, no runtime |
| **Goroutines** | Go's concurrency model is arguably simpler than Tokio. Goroutines are cheap, channels are intuitive, `select` is powerful. For a DAG scheduler running concurrent subprocesses, goroutines are excellent. |
| **Faster compile times** | Go compiles in seconds. Rust compiles in minutes. During iterative development, this matters — a 30-second Rust compile cycle vs a 2-second Go compile cycle changes how you think. |
| **Simpler contributor onboarding** | If Achilles is open-source, community contributors are overwhelmingly security professionals who know Go (because of ProjectDiscovery tools) or Python. Rust's borrow checker is a barrier to contribution. Go's learning curve is 2 weeks; Rust's is 2 months. |
| **Ecosystem alignment** | Writing ADC language bindings for Go is frictionless if Achilles is already Go. The Go ADC binding becomes the reference implementation. If Achilles is Rust, the Go binding is a secondary artifact. |
| **Proven at Achilles's scale** | Docker, Kubernetes, Terraform — Go handles projects far more complex than Achilles. The concurrency model, the standard library, the tooling are battle-tested at scale. |

Go would produce a working Achilles. It would compile faster, onboard contributors easier, and align with the security tool ecosystem. **This is not a straw man — Go is a genuinely strong choice.**

**Why Rust wins anyway — the specific tradeoffs that matter for a security engine:**

**1. Ownership semantics for untrusted input handling.**
Achilles processes untrusted data at every layer: YAML workflow files from community authors, XML output from tool subprocesses, JSON responses from targets. Every piece of data is attacker-influenced. Rust's ownership model guarantees:
- No use-after-free when passing ADC objects between nodes in a concurrent DAG
- No data races when multiple Tokio tasks access the shared state store
- No buffer overflows when parsing malformed XML from a compromised nmap binary (TA-2 in §7.1)

Go's garbage collector prevents memory leaks but does **not** prevent data races. Go has `-race` as a runtime detector, not a compile-time guarantee. For a security tool processing adversarial input, "detected at runtime in testing" is categorically weaker than "rejected at compile time."

**2. Exhaustive pattern matching for the ADC type system.**
The ADC has 7 core types and multiple enums (`Severity`, `Protocol`, `PortState`). Every transformer must handle every variant. In Rust:

```rust
match severity {
    Severity::Critical => ...,
    Severity::High => ...,
    Severity::Medium => ...,
    Severity::Low => ...,
    Severity::Info => ...,
    // Compiler error if you miss one
}
```

In Go, a `switch` on a string or iota constant has no exhaustiveness check. Adding `Severity::None` in ADC v1.1 silently falls through to `default` in every Go switch statement. In Rust, it's a compile error in every `match` — the compiler tells you exactly which functions need updating. For a schema that will evolve (§4.6), this is not a convenience — it is a correctness guarantee.

**3. Wasmtime is a Rust project.**
The WASM sandbox (§6.4, I-02) uses Wasmtime. Wasmtime is written in Rust, exposes a Rust API as its primary interface, and is maintained by the Bytecode Alliance (a Rust-centric organization). The Go bindings for Wasmtime exist but are FFI wrappers — second-class citizens with version lag, missing features, and CGo overhead. Building the sandbox in Rust means using Wasmtime's native API with zero FFI boundary.

**4. Zero-copy deserialization with serde.**
Achilles parses nmap XML files that can exceed 100MB for large network scans. Rust's `quick-xml` + `serde` can parse these with zero-copy deserialization — reading directly from the buffer without allocating intermediate strings. Go's `encoding/xml` allocates for every element. For a transformer processing 50,000 port entries, this is the difference between 200ms and 2 seconds.

**5. The learning signal is real.**
This is a portfolio project for a 2nd-year CS student. A production-quality Rust project with async concurrency, WASM embedding, and type-safe schema validation differentiates from 95% of candidates. A Go project, while technically competent, signals familiarity with a language that most systems engineers already know. Rust signals depth.

**The honest summary:**

| Criterion | Go Wins | Rust Wins |
|---|---|---|
| Compile speed | ✅ | |
| Contributor onboarding | ✅ | |
| Ecosystem alignment with security tools | ✅ | |
| Compile-time memory safety guarantees | | ✅ |
| Exhaustive pattern matching for ADC types | | ✅ |
| Native Wasmtime integration (no FFI) | | ✅ |
| Zero-copy XML/JSON parsing | | ✅ |
| Fearless concurrency (no data races possible) | | ✅ |
| Career differentiation signal | | ✅ |

Go wins on developer experience. Rust wins on correctness guarantees. For a security engine that processes untrusted input, manages concurrent subprocesses, and enforces sandboxing — correctness is the non-negotiable requirement. The developer experience cost is real and honest: slower compiles, harder onboarding, steeper learning curve. The tradeoff is worth it.

### 9.2 Full Technology Stack

```
┌──────────────────────────────────────────────────────────────────┐
│                      ACHILLES TECHNOLOGY STACK                    │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  CORE ENGINE                                                     │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Language:        Rust (2021 edition)                      │  │
│  │  Async Runtime:   Tokio (multi-threaded)                   │  │
│  │  Serialization:   serde + serde_json + serde_yaml          │  │
│  │                   (rmp-serde for MessagePack — future opt) │  │
│  │  XML Parsing:     quick-xml (nmap output)                  │  │
│  │  CLI Framework:   clap v4 (derive API)                     │  │
│  │  Error Handling:  thiserror + anyhow                       │  │
│  │  Logging:         tracing + tracing-subscriber              │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  WORKFLOW ENGINE                                                 │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  DAG Library:     petgraph (graph data structures)         │  │
│  │  Schema Valid:    jsonschema-rs (JSON Schema validation)   │  │
│  │  Expression Lang: Rhai (embedded scripting, sandboxed       │  │
│  │                   ADC subset — see §6.3.1)                 │  │
│  │  Process Mgmt:    tokio::process (async child processes)   │  │
│  │  Cgroups:         cgroups-rs (Linux resource limits)       │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  SANDBOX                                                         │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  WASM Runtime:    Wasmtime (for custom script nodes)       │  │
│  │  WASI Layer:      wasmtime-wasi (capability-based FS/Net) │  │
│  │  SDK Bindings:    wit-bindgen (WASM Interface Types)       │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  STATE & STORAGE                                                 │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  State Store:     SQLite via rusqlite (embedded, no server)│  │
│  │  Secrets Vault:   ring (AES-256-GCM encryption)            │  │
│  │  OS Keyring:      keyring-rs (cross-platform secret store) │  │
│  │  Audit Log:       Append-only file + sha2 (hash chain)    │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  NETWORKING & NOTIFICATIONS                                      │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  HTTP Client:     reqwest (for webhook notifications)      │  │
│  │  DNS:             hickory-dns (async DNS resolution)        │  │
│  │  Slack/Discord:   reqwest + webhook URLs                   │  │
│  │  IP/CIDR:         ipnetwork (CIDR parsing/matching)        │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  CLI & UX                                                        │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  CLI Framework:   clap v4 + clap_complete (autocompletion) │  │
│  │  Terminal UI:     indicatif (progress bars)                │  │
│  │                   console (colors, styling)                │  │
│  │                   dialoguer (interactive prompts)           │  │
│  │  Table Output:    tabled (formatted tables)                │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  TESTING                                                         │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Unit Tests:      Built-in Rust test framework             │  │
│  │  Integration:     assert_cmd + predicates (CLI testing)    │  │
│  │  Snapshot Tests:  insta (snapshot testing for transformers)│  │
│  │  Fuzzing:         cargo-fuzz (AFL-based fuzzing for YAML   │  │
│  │                   parser, transformer inputs)              │  │
│  │  Coverage:        cargo-tarpaulin                          │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  OPTIONAL GUI (Phase 6)                                          │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │  Framework:       Tauri (Rust backend + Web frontend)      │  │
│  │  Frontend:        Svelte (lightweight, reactive)           │  │
│  │  Graph Rendering: D3.js (DAG visualization)                │  │
│  │  Communication:   Tauri IPC (Rust ↔ JS bridge)             │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### 9.3 Why SQLite for State (Not PostgreSQL/Redis)

Achilles is a **single-operator CLI tool**, not a multi-tenant web service. SQLite is the right choice because:

| Criterion | SQLite | PostgreSQL | Redis |
|---|---|---|---|
| **Deployment** | Zero — it's a library linked into the binary | Requires a running server | Requires a running server |
| **Portability** | State is one `.db` file — copy it anywhere | Tied to a server installation | Tied to a server |
| **Performance** | More than sufficient for single-operator workflows (~100K writes/sec) | Overkill | Overkill |
| **Reliability** | ACID transactions, WAL mode for concurrent reads | ACID | Best-effort durability |
| **Footprint** | ~1MB library | ~100MB+ installation | ~10MB installation |
| **Offline** | Works on an airplane, behind a VPN, anywhere | Needs network (if remote) | Needs network (if remote) |

### 9.4 Why Tauri for Optional GUI (Not Electron)

If a GUI is built (Phase 6), **Tauri** is chosen over Electron because:

- **Rust backend:** Achilles is already Rust — Tauri shares the same backend. No need for a Node.js process
- **Binary size:** Tauri app ~10MB vs Electron ~100MB+
- **Memory usage:** ~30MB vs ~200MB+. Critical on assessment laptops with limited RAM
- **Security:** No Node.js — smaller attack surface. The Tauri IPC bridge is explicit and auditable
- **Native webview:** Uses the OS's built-in webview (WebKitGTK on Linux, WebView2 on Windows, WKWebView on macOS) — no bundled Chromium

---

## 10. Implementation Roadmap

The day-by-day implementation roadmap, learning curriculum, and exercises are documented in the companion **Achilles Learning Roadmap** (`docs/achilles_learning_roadmap.md`). This architectural analysis focuses on *what* to build and *why*; the roadmap focuses on *how* and *in what order*. The two documents should be read together — the roadmap references specific sections of this analysis throughout.

### 10.1 Where to Start: The Five Critical Decisions

Before writing a single line of code, an implementer must understand these five architectural decisions in order of importance. Each one constrains everything built on top of it. Getting one wrong early means rewriting later.

**1. The Achilles Data Contract schema (§4)**

Every other component depends on the ADC types. The workflow engine routes typed data. The transformers produce typed data. The scope enforcer validates typed data. The report generator formats typed data. If the `Host`, `Port`, `Finding`, and `Credential` structs are wrong — if they're missing a field, or a field has the wrong type — every transformer, every node, and every test must be rewritten when you fix it. Define the ADC types first. Validate them against real tool output (§4.3) before building anything else.

**2. The Node trait interface (§6.1)**

The `Node` trait is the plugin contract. Every tool node and logic node implements it. The engine only interacts with nodes through this interface. The trait's method signatures — `input_schema()`, `output_schema()`, `execute()`, `validate()` — determine what information the engine has at validation time vs. runtime, what errors are possible, and how data flows between nodes. Get the trait wrong and every node implementation must change.

**3. The scope enforcement model (§5.7)**

Three-point enforcement (pre-workflow, pre-node, post-node) is architecturally expensive — it means the scope enforcer runs before and after every node execution, not just once at startup. This affects the execution loop, the Node trait (nodes must return typed output that the enforcer can inspect), and performance. Decide whether three-point enforcement is required before building the execution loop.

**4. The expression language (§6.3.1)**

Conditional nodes, transform nodes, and split nodes all use expressions. The choice between Rhai, CEL, and a custom DSL affects: what contributors must learn to write logic nodes, what security guarantees you can make about expression evaluation, and how much integration code you write. This decision must be made before implementing any logic node.

**5. The state management schema (§5.4)**

The SQLite schema determines what can be checkpointed, what can be resumed, and what data survives a crash. Once workflows are running and producing state, changing the schema requires a migration. Design the `workflow_runs` and `node_states` tables with all known requirements (including encrypted credential storage from §7.5) before the first workflow executes.

| Week | Deliverable | "Done" Criteria |
|---|---|---|
| 11 | Nuclei transformer + ffuf transformer | Real tool output correctly parsed into `Vec<Finding>` and `Vec<URL>`. 10 test cases each |
| 12 | Sqlmap transformer + DAG scheduler | Sqlmap text output parsed. Parallel node execution via topological sort working |
| 13 | Logic nodes (conditional, merge, split) | Conditional routing, parallel branch merge, and data splitting all functional with tests |
| 14 | Loop node + data transform node | `type: loop` iterates over collections. Transform node converts between ADC types |
| 15 | Approval node (CLI prompt) | Approval node pauses workflow, prints context to terminal, resumes on input. APPROVE / REJECT / APPROVE_PARTIAL |
| 16 | Approval node (webhook) | Slack/Discord webhook notifications. Approval via webhook response. Timeout and escalation |

**Key Risks:**
- Parallel execution race conditions — Mitigate by using Tokio channels with ownership semantics, no shared mutable state
- Approval webhook reliability — Mitigate by implementing multi-channel delivery from the start

**What "Done" looks like:** Full bug bounty recon workflow: subfinder → httpx → [nmap, nuclei, ffuf] (parallel) → approval gate → sqlmap → merge → report. All tools integrated, approval gate functional, parallel execution correct.

### 10.5 Phase 4: Security Hardening (Weeks 17-22)

**Goal:** Implement the security architecture — sandbox, scope enforcement, secrets management.

| Week | Deliverable | "Done" Criteria |
|---|---|---|
| 17 | Scope enforcement engine | Domain/IP/port rules validated at pre-workflow, pre-node, and post-node. DNS resolution check catches shared hosting. 20 test cases |
| 18 | Secrets vault | AES-256-GCM encrypted vault, OS keyring integration, `${secrets.*}` resolution in workflow YAML. Secret values never in logs |
| 19 | Log scrubbing | All node stdout/stderr scrubbed for secret values. Replacement with `[REDACTED:key_name]`. Proven by test showing secret-in-output → scrubbed |
| 20 | WASM sandbox (Wasmtime) | Custom script node executes in Wasmtime. Filesystem/network blocked by default. Capability declarations respected |
| 21 | Audit logging with hash chain | Append-only log with hash chain. Tamper detection via `achilles audit verify`. Every node execution logged |
| 22 | Resource limits (cgroups) | Per-node memory, CPU, time, and network limits enforced. Runaway nodes killed gracefully |

**Key Risks:**
- Wasmtime complexity — Mitigate by starting with a minimal WASM module (hello world), then incrementally adding capabilities
- cgroups requires root or specific capabilities — Mitigate by making cgroups optional (graceful degradation with warnings)

**What "Done" looks like:** Security audit checklist: ✅ Secrets never in logs ✅ Out-of-scope targets blocked ✅ Custom scripts sandboxed ✅ Audit trail tamper-evident ✅ Runaway nodes killed. Document with proof for each.

### 10.6 Phase 5: Template Library + CLI Polish (Weeks 23-28)

**Goal:** Build the Hacker Template Library, polish the CLI experience, workflow sharing.

| Week | Deliverable | "Done" Criteria |
|---|---|---|
| 23 | Bug Bounty Recon template | Complete workflow template with docs. Tested against a real target (personal lab or authorized bug bounty) |
| 24 | Web App Assessment + AD Enumeration templates | Two more templates, each tested in a lab environment |
| 25 | API Security + Network Pentest templates | Two more templates. Total: 5 battle-tested templates |
| 26 | CLI polish — `achilles new`, `achilles graph`, autocompletion | `achilles new` scaffolds a workflow from template. `achilles graph` renders ASCII DAG. Bash/Zsh/Fish autocompletion via `clap_complete` |
| 27 | Workflow validation deep-dive + security review UI | `achilles inspect` shows the full security review (§7.3). Static analysis for shell injection, YAML bombs |
| 28 | `achilles templates share` + community registry | Workflow sharing. Templates uploaded to a community registry (GitHub releases or dedicated repository). Download via `achilles templates download` |

**Key Risks:**
- Template quality requires real-world testing — Mitigate by testing each template in a lab environment (HackTheBox, TryHackMe, personal Vulnhub VMs)
- CLI UX polish is time-consuming — Mitigate by prioritizing the 5 most common commands first

**What "Done" looks like:** A new operator can run `achilles templates download bug_bounty_recon`, fill in the scope, and run a complete recon engagement with zero custom configuration.

### 10.7 Phase 6: Optional GUI + Community (Weeks 29-36)

**Goal:** Build the optional GUI, open-source the project, establish the community contribution pipeline.

| Week | Deliverable | "Done" Criteria |
|---|---|---|
| 29-30 | Tauri app scaffolding + DAG visualization | GUI shows a running workflow as a DAG. Nodes update status in real-time (pending → running → complete) |
| 31-32 | GUI: workflow builder + node palette | Drag-and-drop workflow creation. Export to YAML. Import existing YAML |
| 33 | Community contribution docs + CI | CONTRIBUTING.md, node template generator, CI pipeline for community PR validation |
| 34 | ADC specification publication | ADC v1.0 specification published as a standalone document. JSON Schema files in a dedicated repository |
| 35 | Open-source release preparation | LICENSE (Apache 2.0), README, demo GIFs, installation instructions, GitHub Actions CI |
| 36 | Launch | GitHub release, Reddit/HackerNews posts, security community outreach (BSides, DEFCON Villages) |

**Key Risks:**
- GUI scope creep — Mitigate by strictly limiting GUI to DAG visualization and YAML export. The GUI should never do anything the CLI can't do
- Open-source community management — Mitigate by having a clear CONTRIBUTING.md and issue templates before launch

**What "Done" looks like:** Achilles is live on GitHub with MIT/Apache 2.0 license, 5 workflow templates, 10 tool nodes, a working GUI, and the ADC published as an open specification.

---

## 11. Competitive Landscape

### 11.1 Competitive Matrix

| Dimension | Achilles | Shuffle SOAR | n8n + Security | Tines | Bash Scripts | Nuclei Templates |
|---|---|---|---|---|---|---|
| **CLI-first** | ✅ Primary interface | ❌ GUI-only | ❌ GUI-primary | ❌ GUI-only | ✅ CLI-native | ✅ CLI-native |
| **Open Schema (ADC)** | ✅ Typed ADC objects | ❌ Untyped JSON | ❌ Generic data | ❌ Proprietary | ❌ Raw text/files | ❌ Nuclei-specific JSON |
| **Approval Nodes** | ✅ First-class, multi-channel | ⚠ Webhook-based | ⚠ Manual step | ✅ Built-in | ❌ None | ❌ None |
| **Scope Enforcement** | ✅ Engine-level, 3-point | ❌ None | ❌ None | ❌ None | ❌ Operator discipline | ❌ None |
| **Custom Scripts** | ✅ Sandboxed (WASM) | ⚠ Docker apps (heavy) | ✅ JS functions | ✅ Python/JS | ✅ Any (unsandboxed) | ❌ No custom logic |
| **Portability** | ✅ YAML file, single binary | ❌ Docker required | ❌ Server required | ❌ SaaS only | ⚠ OS-dependent | ✅ Template files |
| **Workflow as Code** | ✅ Version-controlled YAML | ❌ GUI-defined | ⚠ JSON export | ⚠ Proprietary format | ✅ Shell scripts | ✅ YAML templates |
| **Data Normalization** | ✅ Typed transformers | ❌ Manual per-app | ❌ Manual per-node | ❌ Manual | ❌ sed/awk/jq | ❌ Single-tool only |
| **Learning Curve** | 🟠 Medium (YAML + ADC concepts) | 🟡 Low (drag-drop GUI) | 🟡 Low (familiar UI) | 🟡 Low (SaaS) | 🟢 Lowest (bash) | 🟢 Low (YAML) |
| **Audit Trail** | ✅ Cryptographic hash chain | ⚠ Basic logging | ⚠ Execution history | ✅ Detailed logs | ❌ None | ❌ None |
| **Self-contained** | ✅ Single binary | ❌ Docker + DB | ❌ Node.js + DB | ❌ Cloud | ✅ Just bash | ✅ Just nuclei |
| **Cost** | Free (open source) | Free (open source) | Free tier limited | $$$ Enterprise | Free | Free (open source) |
| **Offensive Security Focus** | ✅ Purpose-built | ⚠ SOC/defensive focus | ❌ Generic automation | ⚠ SOC/defensive focus | ✅ Used by pentesters | ✅ Security-focused |

### 11.2 Achilles's Uncontested Niche

> [!IMPORTANT]
> **Achilles occupies a position that no existing tool claims: a CLI-first, typed-schema, scope-enforced workflow engine purpose-built for offensive security.**

The competitive landscape has a clear gap:

```
                        CLI-First
                           ▲
                           │
        Bash Scripts ──────┼────── Achilles
                           │         ★
        Nuclei Templates ──┤
                           │
        ───────────────────┼───────────────────▶ Typed Data Schema
                           │
                    Shuffle ┤
                           │
              n8n ─────────┤
                           │
            Tines ─────────┘
                        GUI-First
```

No tool in the current landscape is:
1. **CLI-first** AND has a **typed data schema** (bash is CLI but untyped; Shuffle has schemas but is GUI)
2. **Offensive security focused** AND has **scope enforcement** (nuclei is offensive but single-tool; Tines has governance but is defensive)
3. **Self-contained** (single binary) AND supports **workflow sharing** (bash is self-contained but scripts aren't portable; n8n supports sharing but requires a server)

Achilles's moat is the **Achilles Data Contract**. If adopted as an open standard, other tools can emit ADC-compatible output, making Achilles the natural integration layer for the entire offensive security toolchain.

---

## 12. Success Metrics & KPIs

### 12.1 Technical Quality Metrics

| Metric | Target (Phase 4 Complete) | Target (Phase 6 Complete) | Measurement |
|---|---|---|---|
| **Workflow execution reliability** | 95% of workflows complete without engine errors | 99% | `(successful_runs / total_runs) * 100` |
| **Data loss rate** | <1% of ADC objects lost between nodes | <0.1% | Compare input count to output count per node |
| **Schema conformance rate** | 100% of node outputs pass ADC validation | 100% | Schema validator pass rate in CI |
| **Transformer correctness** | 95% accuracy on real-world tool output | 99% | Snapshot tests against 50+ real tool outputs per node |
| **Scope enforcement accuracy** | 100% — zero false negatives (never miss a scope violation) | 100% | Fuzzing with edge-case targets |
| **Resume reliability** | 90% of interrupted workflows resume correctly | 99% | Simulate crashes at every node and verify resume |
| **Secret leakage rate** | 0% — secrets never appear in logs, ps, or state | 0% | Automated scan of all output for known secret values |
| **Sandbox escape rate** | 0% — custom scripts cannot access unauthorized resources | 0% | Fuzzing + manual audit of WASM boundary |

### 12.2 Adoption Metrics

| Metric | 6 Months | 12 Months | 24 Months |
|---|---|---|---|
| **GitHub Stars** | 500 | 2,000 | 10,000 |
| **Community Node Contributions** | 3 | 15 | 50 |
| **Template Downloads** | 100 | 1,000 | 10,000 |
| **Monthly Active Users** | 50 | 500 | 5,000 |
| **ADC Adoption** (tools emitting native ADC) | 0 | 2 | 10 |
| **Conference Talks** | 1 (BSides local) | 3 (BSides + regional) | 5 (DEF CON, Black Hat) |

### 12.3 Learning Value Metrics

This project, when completed, demonstrates the following skills to employers:

| Skill | Evidence | Value to Employers |
|---|---|---|
| **Systems programming (Rust)** | Production Rust with async, WASM, cgroups, process management | Top 5% differentiator for junior roles |
| **Type system design** | The Achilles Data Contract — a real schema standard with versioning | Shows API/schema design thinking |
| **Security engineering** | Sandbox, scope enforcement, secrets management, audit logging | Directly relevant for security roles |
| **Infrastructure as Code** | Workflow-as-YAML, DAG execution, state management | Shows DevOps/SRE understanding |
| **Offensive security depth** | Templates for real engagement types, tool integration knowledge | Proves hands-on pentest experience |
| **Open-source contribution** | Public project with community contributions, CI/CD, documentation | Shows collaboration readiness |
| **Technical writing** | This architecture document, ADC specification, CONTRIBUTING.md | Often the weakest skill in junior candidates |

> [!TIP]
> **For a 2nd-year CS student:** A completed Achilles project (especially Phases 1-4) on your resume, with a GitHub repository showing real commits, real tests, and real documentation, is worth more than 10 CTF certifications for landing a security engineering internship.

---

## 13. Conclusion

### 13.1 What Makes Achilles Genuinely Novel

Achilles is not a new wrapper around security tools. The world does not need another `recon.sh`. Achilles is novel for three reasons:

**1. The Achilles Data Contract is a new open standard.**
No one has defined typed, versioned, tool-agnostic data objects for offensive security tooling. STIX/TAXII exists for threat intelligence. SARIF exists for static analysis. OCSF exists for security events. Nothing exists for **pentest tool I/O**. The ADC fills this gap. Even if Achilles the engine is never widely adopted, the ADC as a specification can become the standard interchange format for the offensive security ecosystem — the same way OpenAPI became the standard for REST APIs regardless of which API gateway you use.

**2. Scope enforcement at the engine level is unprecedented.**
No existing workflow automation tool — security-focused or otherwise — performs continuous, engine-level scope enforcement with DNS resolution cross-checking. This is the feature that makes Achilles legally defensible in a way that bash scripts and SOAR platforms are not.

**3. The CLI-first, workflow-as-code paradigm for offensive security doesn't exist.**
Ansible proved that infrastructure automation belongs in YAML files, not GUI dashboards. Achilles makes the same argument for offensive security: your recon methodology should be a version-controlled, peer-reviewed, executable document — not a collection of bash aliases in someone's `.zshrc`.

### 13.2 The Open-Source Imperative

Achilles **must** be open source. This is not a philosophical preference — it is a strategic requirement for three reasons:

1. **Trust.** Security professionals will not run a closed-source binary that executes tools with root-level capabilities, handles credentials, and accesses network targets. The source must be auditable. There is no alternative.

2. **Adoption of the ADC.** An open schema standard must be openly licensed. If the ADC is proprietary, no tool vendor will adopt it. Apache 2.0 license makes adoption frictionless — commercial tools can emit ADC output without legal concern.

3. **Community node contributions.** Achilles's value scales with the number of tool nodes. One team cannot write transformers for every security tool. The community can — but only if the project is open, welcoming, and well-documented.

### 13.3 Final System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                        PROJECT ACHILLES — COMPLETE SYSTEM                    │
│                  The Offensive Security Orchestration Engine                 │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  OPERATOR                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │              CLI  (achilles run / validate / inspect)               │    │
│  │              GUI  (Tauri — optional)                                │    │
│  └─────────────────────────────────┬───────────────────────────────────┘    │
│                                    │                                        │
│                                    ▼                                        │
│  WORKFLOW LAYER                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  ┌──────────┐  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │    │
│  │  │  YAML    │→ │  Schema     │→ │  DAG Builder │→ │   Cycle    │  │    │
│  │  │  Parser  │  │  Validator  │  │  + Type Check│  │  Detector  │  │    │
│  │  └──────────┘  └─────────────┘  └──────────────┘  └────────────┘  │    │
│  └─────────────────────────────────┬───────────────────────────────────┘    │
│                                    │                                        │
│                                    ▼                                        │
│  EXECUTION RUNTIME                                                          │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                                                                     │    │
│  │  ┌─────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │    │
│  │  │   Scope     │  │   Secrets    │  │     DAG Scheduler        │  │    │
│  │  │   Engine    │  │   Vault      │  │  (Tokio async runtime)   │  │    │
│  │  └──────┬──────┘  └──────┬───────┘  └────────────┬─────────────┘  │    │
│  │         │                │                       │                │    │
│  │         ▼                ▼                       ▼                │    │
│  │  ┌──────────────────────────────────────────────────────────────┐ │    │
│  │  │                    NODE EXECUTORS                            │ │    │
│  │  │                                                              │ │    │
│  │  │  ┌──────────────────────────────────────────────────────┐   │ │    │
│  │  │  │  TOOL NODES                                          │   │ │    │
│  │  │  │  subfinder │ httpx │ nmap │ nuclei │ ffuf │ sqlmap   │   │ │    │
│  │  │  │      ↓         ↓       ↓       ↓       ↓       ↓    │   │ │    │
│  │  │  │  [Versioned Transformers → ADC Typed Objects]        │   │ │    │
│  │  │  └──────────────────────────────────────────────────────┘   │ │    │
│  │  │                                                              │ │    │
│  │  │  ┌──────────────────────────────────────────────────────┐   │ │    │
│  │  │  │  LOGIC NODES                                         │   │ │    │
│  │  │  │  conditional │ loop │ merge │ split │ transform      │   │ │    │
│  │  │  └──────────────────────────────────────────────────────┘   │ │    │
│  │  │                                                              │ │    │
│  │  │  ┌──────────────────────────────────────────────────────┐   │ │    │
│  │  │  │  SPECIAL NODES                                       │   │ │    │
│  │  │  │  approval (Slack/Discord/CLI) │ report │ custom_wasm │   │ │    │
│  │  │  └──────────────────────────────────────────────────────┘   │ │    │
│  │  └──────────────────────────────────────────────────────────────┘ │    │
│  │                           │                                       │    │
│  │                           ▼                                       │    │
│  │  ┌──────────────────────────────────────────────────────────────┐ │    │
│  │  │                    STATE & AUDIT                             │ │    │
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │ │    │
│  │  │  │ SQLite State │  │  Audit Log   │  │  WASM Sandbox    │  │ │    │
│  │  │  │ (checkpoints │  │  (hash chain │  │  (Wasmtime +     │  │ │    │
│  │  │  │  + ADC data) │  │   + scrubbed)│  │   capabilities)  │  │ │    │
│  │  │  └──────────────┘  └──────────────┘  └──────────────────┘  │ │    │
│  │  └──────────────────────────────────────────────────────────────┘ │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  DATA CONTRACT (ADC v1.0)                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  Host │ Port │ Service │ URL │ Finding │ Credential │ DNSRecord    │    │
│  │                                                                     │    │
│  │  Open Standard — Apache 2.0 — JSON Schema — Language Bindings      │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
│  TEMPLATE LIBRARY                                                           │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  Bug Bounty │ AD Enum │ Web App │ API Security │ Network Pentest   │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 13.4 Final Words

Achilles is named after the greatest warrior of Greek mythology — but its true power is not in any single tool it wields. Like its namesake, Achilles's strength comes from **how it fights**: disciplined, precise, and relentless.

The single most important thing this project contributes is the **Achilles Data Contract**. If the ADC becomes the open standard for security tool I/O — the way OpenAPI standardized REST APIs, the way SARIF standardized SAST output, the way STIX standardized threat intelligence — then Achilles has changed the industry regardless of whether the engine itself becomes widely adopted.

Build the schema. Build it correctly. Build it openly. Everything else follows.

```
Offensive_Security_Velocity ∝ Schema_Conformance × Automation_Trust
```

---

*Document Version: 1.0*
*Author: Project Achilles Architecture Team*
*Date: February 2026*
*Status: Complete Architectural Analysis*
*Next Step: Begin Phase 1 — Engine POC*
