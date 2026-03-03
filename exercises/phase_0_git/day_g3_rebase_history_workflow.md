# Day G3 — Rebase, History Rewriting, and Professional Workflow

> **Time budget:** 2–4 hours of focused work.
> **Prerequisite:** Day G2 completed. Your `achilles` practice repo is on GitHub with branches, merges, and a resolved conflict.
> **Outcome:** You can rewrite history with interactive rebase, understand rebase vs merge, tag releases, and have the complete professional
Git workflow internalized for all future Achilles development.

> [!IMPORTANT]
> This is the day most tutorials skip. It separates developers who **use** Git from developers who **understand** Git. Everything you learn today will
be used on every single feature branch from Phase 1 onward.

---

## Why This Day Matters

On Day G2 you created branches and merged them. Every merge created a **merge commit** — a commit whose only purpose is to say "I combined these two
branches." For a team of 20 developers, merge commits are valuable historical records. For a solo developer, they are noise.

Look at this history:

```
*   e4f5a6b Merge branch 'phase-0-practice' into main
|\
| * d3c2b1a docs(readme): add architecture overview
| * c2b1a0f feat(engine): add workflow struct definitions
| * b1a0f9e feat(tools): add tool module placeholder
|/
*   9e8d7c6 Merge branch 'conflict-experiment' into main
|\
| * 8d7c6b5 docs(core): update header comment to include version
|/
* 7c6b5a4 docs(core): update header comment to use full project name
* 6b5a4f3 chore: add .gitignore
* 5a4f3e2 docs: add project description to README
```

Those merge commits add visual noise without adding information. **Rebase** eliminates them, producing a clean, linear history:

```
* d3c2b1a docs(readme): add architecture overview
* c2b1a0f feat(engine): add workflow struct definitions
* b1a0f9e feat(tools): add tool module placeholder
* 7c6b5a4 docs(core): update header comment
* 6b5a4f3 chore: add .gitignore
* 5a4f3e2 docs: add project description to README
```

Same code. Same changes. But the history tells a clean story without merge noise. For a solo project like Achilles, this is what you want.

---

## Part 1: What Is Rebase?

Rebase answers the question: **"What if my branch had started from a later point?"**

Instead of creating a merge commit, rebase **replays** your branch's commits on top of another branch — as if you had written them after the latest changes on that branch.

```
┌──────────────────────────────────────────────────────────────────┐
│                    MERGE VS REBASE                                │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  STARTING STATE:                                                 │
│                                                                  │
│     A ← B ← C          (main)                                   │
│          ↖                                                       │
│           D ← E         (feature)                                │
│                                                                  │
│  ─────────────────────────────────────────────────────────       │
│                                                                  │
│  AFTER: git merge feature (on main)                              │
│                                                                  │
│     A ← B ← C ← M      (main)                                  │
│          ↖       ↗                                               │
│           D ← E         merge commit M has two parents           │
│                                                                  │
│  ─────────────────────────────────────────────────────────       │
│                                                                  │
│  AFTER: git rebase main (on feature), then fast-forward merge    │
│                                                                  │
│     A ← B ← C ← D' ← E'   (main, feature)                      │
│                                                                  │
│     D' and E' are NEW commits — same changes as D and E,         │
│     but with new hashes because their parent changed.            │
│     Linear history. No merge commit.                             │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

**The critical insight:** Rebase creates **new commits** (`D'` and `E'`). They have the same diffs as `D` and `E`, but different hashes because
their parent commit changed (from `B` to `C`). The original `D` and `E` still exist in Git's object store but are no longer referenced by any branch —
they'll be garbage-collected eventually.

**The golden rule of rebase:**

> **Never rebase commits that have been pushed to a shared branch that others are working on.**

Rebase rewrites history. If you rebase commits that someone else has already pulled, their history and yours will diverge, and the merge will be a
nightmare. For a **solo project** like Achilles, this is not a concern — you are the only developer. Rebase freely on your feature branches before pushing.

---

## Part 2: Exercises

### Exercise 1 — Rebase a Feature Branch onto Main

**Step 1: Set up the scenario.**

Make sure you're on `main` and it's up to date:

```bash
cd achilles-practice
git checkout main
git pull origin main
```

Create a commit on `main` to simulate it advancing while you work on a branch:

```bash
echo "# Changelog" > CHANGELOG.md
echo "" >> CHANGELOG.md
echo "## v0.0.1 — Phase 0 Practice" >> CHANGELOG.md
echo "- Git fundamentals learned" >> CHANGELOG.md
echo "- Branching and merging practiced" >> CHANGELOG.md
git add CHANGELOG.md
git commit -m "docs: add initial CHANGELOG"
```

**Step 2: Create a feature branch and make commits.**

```bash
git checkout -b phase-0/rebase-practice
```

Make 3 commits on this branch:

```bash
cat > adc.rs << 'EOF'
// Achilles Data Contract — Core Types
// Architecture reference: §4 (The Achilles Data Contract)
//
// These types are the lingua franca between all tool nodes.
// Every datum that flows between tools is one of these types.

pub struct Host {
    pub id: String,
    pub ip_addresses: Vec<String>,
    pub hostnames: Vec<String>,
}
EOF
git add adc.rs
git commit -m "feat(adc): define Host struct with id, IPs, and hostnames"

cat >> adc.rs << 'EOF'

pub struct Port {
    pub number: u16,
    pub protocol: String,
    pub state: String,
    pub service: Option<String>,
}
EOF
git add adc.rs
git commit -m "feat(adc): add Port struct with protocol and state"

cat >> adc.rs << 'EOF'

pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

pub struct Finding {
    pub title: String,
    pub severity: Severity,
    pub description: String,
}
EOF
git add adc.rs
git commit -m "feat(adc): add Severity enum and Finding struct"
```

**Step 3: Observe the divergence.**

```bash
git log --oneline --graph --all
```

You'll see `main` has the CHANGELOG commit, and your branch has 3 ADC commits. They diverged from the commit before the CHANGELOG.

If you merged now, you'd get a merge commit. Instead, let's rebase.

**Step 4: Rebase your branch onto main.**

```bash
git rebase main
```

Output:

```
Successfully rebased and updated refs/heads/phase-0/rebase-practice.
```

**What just happened:** Git took your 3 commits (Host, Port, Finding), temporarily removed them, moved your branch pointer to the tip of `main`
(which includes the CHANGELOG commit), then **replayed** your 3 commits on top. Your commits now sit after the CHANGELOG, as if you had branched AFTER adding the CHANGELOG.

**Step 5: Verify the linear history.**

```bash
git log --oneline --graph --all
```

Now your branch is a straight line extending from `main`. No fork. No merge commit needed.

**Step 6: Fast-forward merge into main.**

```bash
git checkout main
git merge phase-0/rebase-practice
```

Since `main` is an ancestor of your branch (thanks to the rebase), Git does a **fast-forward merge** — it just moves the `main` pointer forward. No merge commit.

```bash
git log --oneline --graph
```

Clean, linear history. Every commit in order. No merge noise.

**Step 7: Clean up.**

```bash
git branch -d phase-0/rebase-practice
```

---

### Exercise 2 — Interactive Rebase: The Most Powerful Git Command

This is where Git goes from useful to transformative. Interactive rebase lets you **rewrite history** — squash multiple commits into one, reword
messages, reorder commits, or drop them entirely.

**Why you need this:** While developing, you commit frequently — including messy commits like "wip", "fix test", "actually fix test", "ok now it works".
Before pushing to GitHub, you clean these up into logical, atomic commits. The public history should look like you got it right the first time.

**Step 1: Create a branch and make messy commits.**

```bash
git checkout -b phase-0/interactive-rebase
```

Simulate a real development session with messy commits:

```bash
cat > audit.rs << 'EOF'
// Audit logging — tamper-evident hash chain
// Architecture reference: §7.4

pub struct AuditEntry {
    pub sequence: u64,
    pub timestamp: String,
    pub event: String,
}
EOF
git add audit.rs
git commit -m "feat(audit): add audit entry struct"

# Oops, forgot a field
cat >> audit.rs << 'EOF'

impl AuditEntry {
    pub fn new(seq: u64, event: String) -> Self {
        AuditEntry {
            sequence: seq,
            timestamp: String::from("TODO"),
            event,
        }
    }
}
EOF
git add audit.rs
git commit -m "wip"

# Fix the timestamp
sed -i 's/String::from("TODO")/chrono::Utc::now().to_rfc3339()/' audit.rs
git add audit.rs
git commit -m "fix timestamp"

# Add hash chain field
sed -i '/pub event: String,/a\    pub prev_hash: String,\n    pub entry_hash: String,' audit.rs
git add audit.rs
git commit -m "add hash fields"

# Update the README
echo "" >> README.md
echo "## Security Features" >> README.md
echo "- Tamper-evident audit log with cryptographic hash chain" >> README.md
git add README.md
git commit -m "update readme"
```

**Step 2: Look at the mess.**

```bash
git log --oneline
```

You'll see something like:

```
f1a2b3c update readme
e9d8c7b add hash fields
d7c6b5a fix timestamp
c5b4a3f wip
b3a2f1e feat(audit): add audit entry struct
```

This is an **embarrassing history**. `"wip"`, `"fix timestamp"`, `"add hash fields"` — meaningless noise. A reviewer learns nothing. Your future self learns nothing.

**Step 3: Interactive rebase to clean it up.**

```bash
git rebase -i HEAD~5
```

`HEAD~5` means "go back 5 commits from HEAD." Git opens your editor with:

```
pick b3a2f1e feat(audit): add audit entry struct
pick c5b4a3f wip
pick d7c6b5a fix timestamp
pick e9d8c7b add hash fields
pick f1a2b3c update readme
```

Each line is a commit. The commands are:

| Command | Short | Effect |
|---------|-------|--------|
| `pick` | `p` | Keep this commit as-is |
| `reword` | `r` | Keep the changes, but rewrite the commit message |
| `squash` | `s` | Merge this commit INTO the previous one (combine changes + messages) |
| `fixup` | `f` | Like squash, but discard this commit's message (keep only the previous message) |
| `drop` | `d` | Delete this commit entirely |
| `edit` | `e` | Pause rebase at this commit, let you amend it |

**Step 4: Edit the rebase plan.**

Change the file to this:

```
pick b3a2f1e feat(audit): add audit entry struct
fixup c5b4a3f wip
fixup d7c6b5a fix timestamp
fixup e9d8c7b add hash fields
reword f1a2b3c update readme
```

**What this does:**
- **Line 1:** Keep the first commit as the base
- **Lines 2-4:** `fixup` — merge "wip", "fix timestamp", and "add hash fields" INTO the first commit. Their code changes are combined, but their
messy messages are discarded. The result is a single commit with the message "feat(audit): add audit entry struct" containing ALL the audit code.
- **Line 5:** `reword` — keep the README changes, but rewrite the vague "update readme" message

Save and close the editor.

Git will then open the editor again for the `reword` commit. Change the message to:

```
docs(readme): add security features section with audit log description
```

Save and close.

**Step 5: Verify the clean history.**

```bash
git log --oneline
```

Now you see:

```
a1b2c3d docs(readme): add security features section with audit log description
f4e5d6c feat(audit): add audit entry struct
```

**Two clean commits** instead of five messy ones. The code is identical — but the history tells a clear story. The audit struct was added in one
logical commit. The docs were updated in another.

**Step 6: Push and clean up.**

```bash
git checkout main
git merge phase-0/interactive-rebase
git branch -d phase-0/interactive-rebase
git push origin main
```

---

### Exercise 3 — Handling a Rebase Conflict

Rebases can hit conflicts, just like merges. The process is slightly different.

**Step 1: Create a conflict scenario.**

```bash
# Create a branch and edit the first line of adc.rs
git checkout -b phase-0/rebase-conflict
sed -i '1s/.*/\/\/ ADC — Achilles Data Contract v1.0/' adc.rs
git add adc.rs
git commit -m "docs(adc): update header to include version"

# Go back to main and edit the same line differently
git checkout main
sed -i '1s/.*/\/\/ Achilles Data Contract — Core Type Definitions/' adc.rs
git add adc.rs
git commit -m "docs(adc): clarify header comment"
```

**Step 2: Attempt the rebase.**

```bash
git checkout phase-0/rebase-conflict
git rebase main
```

Output:

```
CONFLICT (content): Merge conflict in adc.rs
error: could not apply <hash>... docs(adc): update header to include version
hint: Resolve all conflicts manually, mark them as resolved with
hint: "git add <pathspec>..." and run "git rebase --continue".
```

**Step 3: Resolve the conflict.**

The process is nearly identical to merge conflict resolution, but instead of `git commit`, you use `git rebase --continue`:

```bash
# 1. Open the file and fix the conflict markers
#    (same <<<<<<< / ======= / >>>>>>> markers as Day G2)
nano adc.rs    # or your editor of choice

# 2. Choose the version you want (or combine them)
#    Remove ALL conflict markers

# 3. Stage the resolved file
git add adc.rs

# 4. Continue the rebase (NOT git commit)
git rebase --continue
```

> **Key difference from merge conflicts:**
> - Merge conflict → resolve → `git commit`
> - Rebase conflict → resolve → `git add` → `git rebase --continue`
>
> If things go wrong and you want to abort, use `git rebase --abort` — this restores everything to the state before you started the rebase.

**Step 4: Clean up.**

```bash
git checkout main
git merge phase-0/rebase-conflict
git branch -d phase-0/rebase-conflict
```

---

### Exercise 4 — Tagging Releases

Tags mark specific commits as release points. Unlike branches, tags don't move — they permanently point to one commit.

**Step 1: Create an annotated tag.**

```bash
git tag -a v0.0.1 -m "Phase 0 complete: Git fundamentals, branching, rebasing"
```

- `-a` — Annotated tag (stores tagger name, date, message — use this for releases)
- `v0.0.1` — Tag name, following [Semantic Versioning](https://semver.org): `MAJOR.MINOR.PATCH`
- `-m` — Tag message

**Step 2: View the tag.**

```bash
git show v0.0.1
```

This shows the tag metadata AND the commit it points to.

```bash
git log --oneline --decorate
```

You'll see `(tag: v0.0.1)` next to the tagged commit.

**Step 3: Push the tag to GitHub.**

```bash
git push origin v0.0.1
```

Tags don't push automatically with `git push`. You must push them explicitly. On GitHub, go to your repository → "Releases" tab — your tag is there.
You can later create a GitHub Release from this tag with release notes, changelogs, and binary downloads.

> **Semantic Versioning for Achilles:**
>
> | Version | Meaning | When |
> |---------|---------|------|
> | `v0.1.0` | First feature milestone — subprocess runner works | After Phase 1 |
> | `v0.2.0` | ADC types and transformers complete | After Phase 2 |
> | `v0.3.0` | Async pipeline with DAG execution | After Phase 3 |
> | `v0.5.0` | Workflow engine end-to-end | After Phase 4 |
> | `v0.8.0` | Security, CLI, and nodes complete | After Phase 7 |
> | `v1.0.0` | Public release — tested, documented, battle-ready | After Phase 10 |

**Step 4: Push all tags at once (for future reference).**

```bash
# Push ALL local tags to remote
git push origin --tags
```

---

### Exercise 5 — Create the Achilles `.gitignore`

Before Phase 1 begins, you need a `.gitignore` that prevents sensitive and generated files from ever entering the repository.

If you already created a basic `.gitignore` in Day G1, **replace its contents** with this comprehensive version:

```bash
cat > .gitignore << 'EOF'
# ─── Rust Build Artifacts ───────────────────────────────
/target                  # Cargo build output — never commit
debug/                   # Debug build artifacts
*.pdb                    # Windows debug symbols

# ─── Achilles Runtime Files ─────────────────────────────
*.enc                    # Encrypted vault files — NEVER commit secrets
*.log                    # Audit logs — runtime artifacts, not source
.env                     # Environment variables — NEVER commit secrets
.env.*                   # Environment variants (.env.local, .env.prod)
achilles.db              # SQLite state database — runtime data
achilles.db-journal      # SQLite journal
achilles.db-wal          # SQLite write-ahead log

# ─── Scan Output & Reports ──────────────────────────────
/output/                 # Default scan output directory
/reports/                # Generated reports
*.xml                    # Raw nmap XML output (large, regeneratable)
*.jsonl                  # Raw tool JSONL output

# ─── IDE & Editor Files ─────────────────────────────────
.idea/                   # JetBrains IDEs
.vscode/                 # VS Code (except shared settings if desired)
*.swp                    # Vim swap files
*~                       # Emacs backup files
\#*\#                    # Emacs auto-save files
.dir-locals.el           # Emacs directory-local variables

# ─── OS Files ───────────────────────────────────────────
.DS_Store                # macOS Finder metadata
Thumbs.db                # Windows thumbnail cache
EOF

git add .gitignore
git commit -m "chore: add comprehensive .gitignore for Rust, Achilles runtime, and editor files"
git push origin main
```

**Understand each section:**
- **Rust build artifacts:** The `target/` directory is generated by `cargo build`. It can be gigabytes. Never commit it.
- **Achilles runtime files:** Encrypted secrets (`.enc`), audit logs (`.log`), environment variables (`.env`), and the SQLite database are all runtime
artifacts. Committing secrets is a security incident. Committing logs and databases pollutes the repo.
- **Scan output:** Raw tool output is regeneratable — commit the code that generates it, not the output itself.
- **Editor files:** Swap files, backup files, IDE configs. Personal to each developer.
- **Emacs files:** `*~` backup files and `#*#` auto-save files — since you're an Emacs user, these are particularly important.

---

## Part 2b: Fixing Mistakes — Every Undo Operation You Need

Git has an undo for everything. The problem is knowing *which* undo. This section covers every scenario you'll encounter.

```
┌──────────────────────────────────────────────────────────────────┐
│                    GIT UNDO CHEAT SHEET                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  WHAT WENT WRONG                       FIX                       │
│  ─────────────────────────────────     ──────────────────────    │
│  Wrong commit message (last commit)    git commit --amend        │
│  Forgot to add a file (last commit)    git add file; --amend     │
│  Want to undo last commit (keep code)  git reset --soft HEAD~1   │
│  Want to undo last commit (restage)    git reset --mixed HEAD~1  │
│  Want to nuke last commit entirely     git reset --hard HEAD~1   │
│  Undo a commit on public history       git revert <hash>         │
│  Accidentally staged a file            git restore --staged file │
│  Want to discard changes in a file     git restore file           │
│  Need to switch branches, code dirty   git stash                 │
│  Did something catastrophic            git reflog                │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### Exercise 6 — Amending the Last Commit

The most common mistake: you committed with a typo in the message, or you forgot to include a file.

**Scenario A: Fix a wrong commit message.**

```bash
# Make a commit with a bad message
echo "// TODO: implement scope validation" >> scope.rs
git add scope.rs
git commit -m "add tdo comment"
```

That message has a typo and doesn't follow the convention. Fix it:

```bash
git commit --amend -m "feat(scope): add TODO placeholder for scope validation"
```

**What happened:** `--amend` replaces the last commit with a new one. Same code changes, new message. The old commit is gone from history (it still
exists in Git's object store temporarily, but no branch points to it).

**Scenario B: Forgot to include a file.**

```bash
# You committed but forgot to add a file
echo "// Scope validation tests" > scope_test.rs
# Oops — you committed scope.rs but forgot scope_test.rs

# Add the forgotten file and amend
git add scope_test.rs
git commit --amend --no-edit
```

`--no-edit` keeps the existing commit message. The amended commit now includes both `scope.rs` AND `scope_test.rs`.

> [!WARNING]
> `git commit --amend` **rewrites the last commit** — it gets a new hash. If you've already pushed this commit, you'll need to force-push (`git push --force-with-lease`),
which is fine on your own feature branch but **never on `main` when others have pulled**.

---

### Exercise 7 — Reset: Moving HEAD Backwards

`git reset` moves your branch pointer backwards in history. It has three modes, and understanding the difference is critical.

**Step 1: Create some commits to work with.**

```bash
echo "line 1" > reset_test.txt
git add reset_test.txt
git commit -m "test: add line 1"

echo "line 2" >> reset_test.txt
git add reset_test.txt
git commit -m "test: add line 2"

echo "line 3" >> reset_test.txt
git add reset_test.txt
git commit -m "test: add line 3"
```

Now you have 3 new commits. Let's undo them in different ways.

**`--soft`: Undo the commit, keep everything staged.**

```bash
git reset --soft HEAD~1
```

```
Before:  commit A ← commit B ← commit C (HEAD)
After:   commit A ← commit B (HEAD)
         "line 3" is STILL in your file AND still staged
```

Check: `git status` — the changes from "line 3" are staged ("Changes to be committed"). The file still contains "line 3". You can re-commit with a
better message or add more changes before committing.

**Use case:** "I want to redo the last commit — maybe combine it with other changes or fix the message."

Let's re-commit to restore our state:

```bash
git commit -m "test: add line 3"
```

**`--mixed` (the default): Undo the commit, unstage changes, keep them in working directory.**

```bash
git reset --mixed HEAD~1
# same as: git reset HEAD~1  (--mixed is the default)
```

```
Before:  commit A ← commit B ← commit C (HEAD)
After:   commit A ← commit B (HEAD)
         "line 3" is STILL in your file but NOT staged
```

Check: `git status` — the changes are in your working directory ("Changes not staged for commit") but NOT in the staging area. You need to `git add` them again to recommit.

**Use case:** "I want to undo the last commit and rethink which changes go into which commit."

Re-add and recommit:

```bash
git add reset_test.txt
git commit -m "test: add line 3"
```

**`--hard`: Undo the commit AND destroy the changes. Gone.**

```bash
git reset --hard HEAD~1
```

```
Before:  commit A ← commit B ← commit C (HEAD)
After:   commit A ← commit B (HEAD)
         "line 3" is GONE from the file, staging area, and history
```

Check: `cat reset_test.txt` — only "line 1" and "line 2". The changes are **deleted**.

> [!CAUTION]
> `git reset --hard` **destroys uncommitted changes permanently.** There is no undo for this (unless you use `git reflog` — covered in Exercise 9).
Use it only when you're certain you want to throw away work.

**`HEAD~N` explained:**

| Reference | Meaning |
|-----------|---------|
| `HEAD~1` | One commit before HEAD (the second-latest) |
| `HEAD~2` | Two commits before HEAD |
| `HEAD~5` | Five commits back |
| `HEAD^` | Same as `HEAD~1` (alternative syntax) |
| `abc123f` | A specific commit hash — reset to *that exact commit* |

**Going back multiple commits:**

```bash
# Undo the last 3 commits but keep all changes staged
git reset --soft HEAD~3

# Undo the last 3 commits, keep files as-is but unstaged
git reset HEAD~3

# Nuke the last 3 commits completely
git reset --hard HEAD~3
```

**Summary of the three reset modes:**

```
┌──────────┬──────────────┬───────────────┬──────────────────┐
│ Mode     │ Commit gone? │ Changes staged│ Changes in file? │
├──────────┼──────────────┼───────────────┼──────────────────┤
│ --soft   │ Yes          │ Yes           │ Yes              │
│ --mixed  │ Yes          │ No            │ Yes              │
│ --hard   │ Yes          │ No            │ NO — DELETED     │
└──────────┴──────────────┴───────────────┴──────────────────┘
```

---

### Exercise 8 — Revert: Undoing a Commit in Public History

`git reset` rewrites history — it removes commits. This is fine on your own branch before pushing. But what if you need to undo a commit that's already
on `main` and potentially pulled by others (or that you want a record of)?

`git revert` creates a **new commit** that undoes the changes of a previous commit, without removing it from history.

**Step 1: Create a commit you'll want to undo.**

```bash
echo "DANGEROUS SETTING: allow_out_of_scope=true" >> config.rs
git add config.rs
git commit -m "feat(config): add scope bypass setting"
git push origin main
```

This commit is now public. You realize it introduces a dangerous feature that shouldn't exist.

**Step 2: Revert it.**

```bash
git revert HEAD
```

Git opens your editor with a default message like:

```
Revert "feat(config): add scope bypass setting"

This reverts commit abc123f.
```

You can customize the message or accept the default. Save and close.

**Step 3: Observe the result.**

```bash
git log --oneline -3
```

```
d4e5f6a Revert "feat(config): add scope bypass setting"
abc123f feat(config): add scope bypass setting
9876543 previous commit...
```

Both commits exist in history. The revert commit **undoes** the change, but the original is still visible for audit purposes. This is why `revert` is preferred
over `reset` for public/pushed history.

Check the file:

```bash
cat config.rs
```

The "DANGEROUS SETTING" line is gone. The revert removed it cleanly.

**Reverting a specific older commit:**

```bash
# Revert a commit that's not the latest
git revert <commit-hash>
```

Git creates a new commit that undoes just that specific commit's changes, even if other commits came after it.

> **Reset vs Revert — when to use which:**
>
> | Scenario | Tool | Why |
> |----------|------|-----|
> | Undo last commit on your **unpushed** branch | `git reset` | Clean — removes the commit, no trace |
> | Undo a commit that's already **pushed/public** | `git revert` | Safe — creates a new commit, preserves history |
> | Undo a commit and want an **audit trail** | `git revert` | The revert commit documents *why* it was undone |

---

### Exercise 9 — Stash, Restore, and the Safety Net (Reflog)

Three more essential operations that complete your Git recovery toolkit.

**Part A: Unstaging and discarding changes.**

```bash
# Make some changes
echo "// temporary debug code" >> main.rs
git add main.rs

# Oops — I didn't mean to stage that
git restore --staged main.rs
# The file is STILL modified, but no longer staged

# Actually, I want to discard the change entirely
git restore main.rs
# The file is now back to the last committed version
```

> **Modern vs legacy syntax:**
>
> | Operation | Modern (use this) | Legacy (you'll see in old docs) |
> |-----------|-------------------|---------------------------------|
> | Unstage a file | `git restore --staged file` | `git reset HEAD file` |
> | Discard changes | `git restore file` | `git checkout -- file` |
>
> The `git restore` command was introduced in Git 2.23 to make these operations less confusing. Use it.

**Part B: Stash — parking your work temporarily.**

You're halfway through editing `scope.rs` and need to switch branches urgently. The changes aren't ready to commit. Stash saves them:

```bash
# You have uncommitted changes
echo "// work in progress — scope validation logic" >> scope.rs

# Stash them — working directory becomes clean
git stash push -m "WIP: scope validation logic"

# Now you can switch branches, do other work
git checkout main
# ... do whatever you need ...
git checkout -    # go back to previous branch

# Restore your stashed work
git stash pop
# scope.rs has your changes back
```

**Stash commands you need:**

| Command | What it does |
|---------|-------------|
| `git stash push -m "description"` | Save current changes with a label |
| `git stash list` | Show all stashes |
| `git stash pop` | Restore the most recent stash and remove it from the stash list |
| `git stash apply` | Restore the most recent stash but keep it in the stash list |
| `git stash drop` | Delete the most recent stash without applying it |
| `git stash pop stash@{2}` | Restore a specific stash by index |

**Part C: Reflog — the safety net for everything.**

`git reflog` is Git's flight recorder. It records **every** position HEAD has pointed to — even commits that are no longer referenced by any branch. If
you `reset --hard` and lose commits, reflog can save you.

```bash
git reflog
```

Output:

```
d4e5f6a HEAD@{0}: revert: Revert "feat(config): add scope bypass setting"
abc123f HEAD@{1}: commit: feat(config): add scope bypass setting
9876543 HEAD@{2}: reset: moving to HEAD~1
f1a2b3c HEAD@{3}: commit: test: add line 3
e9d8c7b HEAD@{4}: commit: test: add line 2
...
```

Every action is logged with an index. If you lose a commit — even after `reset --hard` — you can find its hash in reflog and recover it:

```bash
# "Oh no, I reset --hard and lost my work!"
git reflog
# Find the commit hash before the reset

# Recover it
git reset --hard HEAD@{3}
# OR create a branch pointing to it:
git branch recovery-branch HEAD@{3}
```

> [!TIP]
> Reflog entries expire after 90 days by default. But within that window, **almost nothing is truly lost in Git**. This is your emergency parachute.

**Exercise: Prove reflog works.**

```bash
# Create a commit
echo "reflog test" > reflog_test.txt
git add reflog_test.txt
git commit -m "test: reflog recovery exercise"

# Note the hash
git log --oneline -1

# Destroy it
git reset --hard HEAD~1

# It's gone from git log
git log --oneline -3
# reflog_test.txt is gone from the working directory

# But reflog remembers
git reflog -5
# Find the hash of the destroyed commit

# Recover it
git reset --hard HEAD@{1}

# It's back!
cat reflog_test.txt
git log --oneline -1
```

Clean up the test file when done:

```bash
git reset --hard HEAD~1
rm -f reflog_test.txt reset_test.txt scope_test.rs
```

---

## Part 3: The Complete Professional Workflow

This is the workflow you will use for **every feature from Phase 1 onward**. It incorporates everything from Days G1–G3.

```
┌──────────────────────────────────────────────────────────────────┐
│          THE ACHILLES DEVELOPMENT WORKFLOW                        │
│          (Every feature, every phase, every time)                 │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1.  git checkout main && git pull origin main                   │
│      Get the latest stable code.                                 │
│                                                                  │
│  2.  git checkout -b phase-N/short-description                   │
│      Create a branch. Name tells you the phase and feature.      │
│                                                                  │
│  3.  Write code. Commit frequently. Messy is OK here.            │
│      "wip", "fix test", "try different approach" — all fine.     │
│      The branch is YOUR workspace.                               │
│                                                                  │
│  4.  When the feature works:                                     │
│      git rebase -i HEAD~N                                        │
│      Clean up WIP commits. Squash, fixup, reword.                │
│      The result: clean, atomic, well-messaged commits.           │
│                                                                  │
│  5.  git rebase main                                             │
│      Replay your clean commits on top of latest main.            │
│      (Resolve conflicts if any.)                                 │
│                                                                  │
│  6.  git push origin phase-N/short-description                   │
│      Push the clean branch.                                      │
│                                                                  │
│  7.  Open a Pull Request on GitHub.                              │
│      Title: type(component): description                         │
│      Body: what, why, how to test.                               │
│                                                                  │
│  8.  Review your own diff.                                       │
│      Read every line. Would a stranger understand?               │
│      Catch dead code, unclear names, missing tests.              │
│                                                                  │
│  9.  Merge the PR (squash-and-merge or rebase-and-merge).        │
│                                                                  │
│ 10.  git checkout main && git pull origin main                   │
│      git branch -d phase-N/short-description                     │
│      Clean up. Ready for the next feature.                       │
│                                                                  │
│  BRANCH NAMING EXAMPLES:                                         │
│    phase-1/subprocess-runner                                     │
│    phase-2/adc-host-struct                                       │
│    phase-3/tokio-async-runner                                    │
│    phase-4/dag-builder                                           │
│    phase-6/scope-enforcer                                        │
│    phase-7/nmap-node                                             │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

### GitHub PR Merge Strategy

When merging PRs on GitHub, you have three options. **Set your preference now:**

| Strategy | What it does | When to use |
|----------|-------------|-------------|
| **Create a merge commit** | Creates a merge commit. Preserves all branch commits. | Team projects where branch history matters |
| **Squash and merge** | Combines all branch commits into ONE commit on main. | When you have many small commits and want one clean entry |
| **Rebase and merge** | Replays branch commits on main individually. No merge commit. | **Recommended for Achilles** — clean linear history with each commit preserved |

**For Achilles, use "Rebase and merge"** — it preserves your individual clean commits (that you already cleaned with `git rebase -i`) without creating
merge commits. You can set this as the default in GitHub: Repository → Settings → General → Pull Requests → select "Allow rebase merging", deselect others if you want to enforce it.

---

## Part 4: Key Concepts to Internalize

| Question | Your Answer Should Include |
|----------|--------------------------|
| What does rebase do? | Replays your commits on top of another branch — rewrites history to create linear timeline |
| How is rebase different from merge? | Merge preserves both branches with a merge commit. Rebase rewrites commits to create linear history. |
| What is `git rebase -i HEAD~5`? | Interactive rebase on the last 5 commits — lets you squash, reword, drop, reorder |
| What does `squash` do? | Combines a commit with the one before it (merges both changes and messages) |
| What does `fixup` do? | Like squash, but discards the commit's message (keeps only the parent's message) |
| What's the golden rule of rebase? | Never rebase commits already pushed to a shared branch others are working on |
| How do you resolve a rebase conflict? | Fix markers → `git add` → `git rebase --continue` (NOT `git commit`) |
| How do you abort a bad rebase? | `git rebase --abort` — restores everything to pre-rebase state |
| How do you fix the last commit message? | `git commit --amend -m "new message"` |
| How do you add a forgotten file to the last commit? | `git add file` → `git commit --amend --no-edit` |
| What are the three reset modes? | `--soft` (keep staged), `--mixed` (keep in working dir), `--hard` (delete everything) |
| When do you use `reset` vs `revert`? | `reset` for unpushed history (removes commit), `revert` for pushed history (new commit that undoes) |
| How do you unstage a file? | `git restore --staged file` |
| How do you discard uncommitted changes? | `git restore file` |
| What is `git stash`? | Saves uncommitted changes temporarily so you can switch context, then `git stash pop` to restore |
| What is `git reflog`? | Git's flight recorder — logs every HEAD position, lets you recover "lost" commits after reset |
| What is an annotated tag? | A permanent pointer to a commit with metadata (tagger, date, message) — used for releases |
| What is Semantic Versioning? | `MAJOR.MINOR.PATCH` — major = breaking changes, minor = new features, patch = bug fixes |
| What's the recommended PR merge strategy for Achilles? | Rebase and merge — clean linear history, individual commits preserved |

---

## Completion Checklist

Before moving to Phase 0b (Lab Setup), verify all of the following:

- [X] I rebased a feature branch onto `main` and observed the linear history (no merge commit)
- [X] I used `git rebase -i HEAD~N` to squash/fixup messy commits into clean ones
- [X] I used `reword` to fix a commit message during interactive rebase
- [X] I resolved a conflict during rebase using `git add` → `git rebase --continue`
- [X] I used `git commit --amend` to fix a commit message and to add a forgotten file
- [X] I used all three `git reset` modes (`--soft`, `--mixed`, `--hard`) and understand the difference
- [X] I used `git revert` to undo a pushed commit with a new revert commit
- [X] I used `git restore --staged` and `git restore` to unstage and discard changes
- [X] I used `git stash` to save and restore work-in-progress changes
- [X] I used `git reflog` to recover a commit destroyed by `git reset --hard`
- [X] I created an annotated tag `v0.0.1` and pushed it to GitHub
- [X] I have a comprehensive `.gitignore` covering Rust, Achilles runtime, and editor files
- [X] I can explain the 10-step professional workflow from memory
- [X] I understand when to use rebase vs merge, and why Achilles prefers rebase
- [X] My GitHub repository has clean, linear history visible in the commit log

---

## Lesson

> Your GitHub profile is your resume. Employers, conference reviewers, and open-source contributors will look at your commit history, your branch
structure, your PR descriptions, and your CI status before they look at your code. A clean, well-structured repository signals that you understand
software development as a collaborative, disciplined practice — not just as writing code that runs.

---

## Phase 0 Complete 🎉

You now have the Git skills to work professionally. Every concept — three-zone model, branching, merging, conflict resolution, rebase, interactive
history rewriting, amending, resetting, reverting, stashing, tagging — will be used every day from Phase 1 onward.

**Next: Phase 0b — Lab Environment Setup →**
