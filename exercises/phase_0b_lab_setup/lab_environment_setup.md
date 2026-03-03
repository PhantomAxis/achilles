# Phase 0b — Lab Environment Setup

> **Time budget:** 2–4 hours of focused work.
> **Prerequisite:** Phase 0 complete. Git is mastered. Your practice repo is on GitHub.
> **Outcome:** Rust toolchain installed, all five security tools working, a legal target environment ready, and the real Achilles project initialized.

> [!IMPORTANT]
> You cannot learn offensive security tools on systems you don't own. Every tool installed today will be wrapped by an Achilles node in later phases. If a tool isn't 
> installed, the node can't run. Get this right now.

---

## Why This Day Matters

On Day 3 you'll write Rust code that spawns `nmap` as a subprocess and parses its XML output. On Day 23 you'll build the full nmap node. 
On Day 24, subfinder. On Day 25, httpx. If any of these tools aren't installed and working, you'll be debugging installation problems instead of writing code.

Professional pentesters set up lab environments before every engagement. Tool installation, target availability, and permission verification 
happen before a single packet is sent. Build this discipline now.

---

## Part 1: Install the Rust Toolchain

Rust is the language for Achilles. Every line of the engine — subprocess management, ADC types, workflow engine, DAG executor, scope enforcer — is Rust.

### Step 1: Install Rust via rustup

On Arch Linux, you have two options. **Use rustup** (not the `rust` pacman package) — it gives you control over toolchain versions, components, and targets.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, choose option **1** (default installation).

After installation, source the environment:

```bash
source "$HOME/.cargo/env"
```

Add this to your shell config permanently. For **zsh** (`~/.zshrc`):

```bash
echo 'source "$HOME/.cargo/env"' >> ~/.zshrc
```

For **bash** (`~/.bashrc`):

```bash
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
```

> **Why rustup over pacman?**
>
> `sudo pacman -S rust` installs a system-wide Rust version managed by Arch's package manager. This is fine for casual use, but:
> - You can't easily switch between stable/nightly toolchains
> - You can't install specific Rust versions for compatibility
> - `rustup` manages components (clippy, rustfmt, rust-analyzer) individually
>
> For a project as serious as Achilles, use rustup. It's the official Rust installer recommended by the Rust project itself.

### Step 2: Verify the installation

```bash
rustc --version
cargo --version
rustup --version
```

You should see version numbers for all three. As of writing, stable Rust is 1.85+.

### Step 3: Install essential Cargo tools

```bash
# Auto-rebuild on file change — invaluable during development
cargo install cargo-watch

# Linter — catches common mistakes and suggests idiomatic Rust
rustup component add clippy

# Formatter — enforces consistent code style
rustup component add rustfmt

# Language server — powers IDE features (autocomplete, go-to-definition, etc.)
rustup component add rust-analyzer
```

**Verify each:**

```bash
cargo watch --version
cargo clippy --version
cargo fmt --version
rust-analyzer --version
```

### Step 3b: Set up Emacs for Rust development

You have two options. **Pick one, not both** — they conflict with each other.

---

#### Option 1: Rustic (Recommended — All-in-One)

`rustic` bundles everything: syntax highlighting, LSP integration (rust-analyzer), cargo commands, inline compilation errors, format-on-save, and more. One package, zero wiring.

**Install with `use-package`** (add to your `~/.emacs.d/init.el` or equivalent):

```elisp
(use-package rustic
  :ensure t
  :config
  ;; Use eglot as the LSP client (built-in since Emacs 29)
  (setq rustic-lsp-client 'eglot)

  ;; Format on save (optional but recommended)
  (setq rustic-format-on-save t)

  ;; Keybindings (rustic sets these up, but here's what you get):
  ;; C-c C-c C-r  →  cargo run
  ;; C-c C-c C-b  →  cargo build
  ;; C-c C-c C-t  →  cargo test
  ;; C-c C-c C-k  →  cargo clippy
  ;; C-c C-c C-f  →  rustfmt (format buffer)
  )
```

**Install without `use-package`:**

```
M-x package-install RET rustic RET
```

Then add to your init file:

```elisp
(require 'rustic)
(setq rustic-lsp-client 'eglot)
(setq rustic-format-on-save t)
```

**Verify it works:** Open any `.rs` file. You should see:
- Syntax highlighting immediately
- After a few seconds, `eglot` connects to `rust-analyzer` (check the modeline — it should show `[eglot:rust-analyzer]`)
- Hover over a type or function for documentation
- `M-.` to jump to definition, `M-,` to jump back
- `C-c C-c C-r` to run `cargo run`
- Inline error squiggles as you type

---

#### Option 2: Manual Setup (rust-mode + eglot)

If you prefer assembling your own setup piece by piece:

```elisp
;; 1. rust-mode — syntax highlighting and basic cargo integration
(use-package rust-mode
  :ensure t
  :hook (rust-mode . eglot-ensure)  ;; auto-connect to rust-analyzer
  :config
  (setq rust-format-on-save t))

;; 2. eglot is built-in since Emacs 29 — no install needed
;; It connects to rust-analyzer automatically via rust-mode hook above

;; 3. Optional: cargo.el for running cargo commands
(use-package cargo
  :ensure t
  :hook (rust-mode . cargo-minor-mode))
;; Adds: C-c C-c C-b (build), C-c C-c C-r (run), C-c C-c C-t (test)
```

**Without `use-package`:**

```
M-x package-install RET rust-mode RET
```

```elisp
(require 'rust-mode)
(add-hook 'rust-mode-hook 'eglot-ensure)
(setq rust-format-on-save t)
```

---

> [!TIP]
> **Which should you pick?** If you want things to just work with minimal config, use **rustic**. If you like building your editor setup from scratch and tweaking
every piece, use **rust-mode + eglot**. Both use `rust-analyzer` under the hood — the IDE features (completions, diagnostics, go-to-definition) are identical.

### Step 4: Test with a throwaway project

```bash
cd /tmp
cargo new rust-test && cd rust-test
```

Edit `src/main.rs`:

```bash
cat > src/main.rs << 'EOF'
fn main() {
    let tools = vec!["nmap", "subfinder", "httpx", "nuclei", "ffuf"];

    println!("Achilles will orchestrate {} tools:", tools.len());
    for (i, tool) in tools.iter().enumerate() {
        println!("  {}. {}", i + 1, tool);
    }

    // This is a preview of what Achilles does — spawn subprocesses
    let output = std::process::Command::new("echo")
        .arg("Hello from a subprocess!")
        .output()
        .expect("Failed to execute subprocess");

    println!("\nSubprocess output: {}", String::from_utf8_lossy(&output.stdout));
}
EOF
```

Build and run:

```bash
cargo run
```

Expected output:

```
Achilles will orchestrate 5 tools:
  1. nmap
  2. subfinder
  3. httpx
  4. nuclei
  5. ffuf

Subprocess output: Hello from a subprocess!
```

Try the development tools:

```bash
# Run the linter
cargo clippy

# Format the code
cargo fmt

# Auto-rebuild on change (Ctrl+C to stop)
cargo watch -x run
```

If all of this works, your Rust toolchain is ready. Clean up:

```bash
cd ~
rm -rf /tmp/rust-test
```

---

## Part 2: Install Security Tools

These are the five tools Achilles will orchestrate. Each one becomes a node in the Achilles engine (Phase 7, Days 23–27).

| Tool | Purpose | Achilles Node (Phase 7) | Language |
|------|---------|------------------------|----------|
| **nmap** | Port scanning, service detection | Day 23 | C |
| **subfinder** | Subdomain enumeration | Day 24 | Go |
| **httpx** | HTTP probing, tech detection | Day 25 | Go |
| **nuclei** | Vulnerability scanning with templates | Day 26 | Go |
| **ffuf** | Web fuzzing (directories, parameters) | Day 27 | Go |

### Step 1: Install nmap

nmap is in the official Arch repositories:

```bash
sudo pacman -S nmap
```

Verify:

```bash
nmap --version
```

**Understand privileges:**

```bash
# TCP Connect scan — no root needed
nmap -sT scanme.nmap.org -p 22,80,443

# SYN scan — requires root (sends raw packets)
sudo nmap -sS scanme.nmap.org -p 22,80,443
```

| Scan Type | Flag | Root Required | Why |
|-----------|------|--------------|-----|
| TCP Connect | `-sT` | No | Completes the full TCP handshake via the OS |
| SYN Scan | `-sS` | Yes | Sends raw SYN packets — requires raw socket access |
| UDP Scan | `-sU` | Yes | Sends raw UDP packets |
| OS Detection | `-O` | Yes | Requires raw packet analysis |

**Achilles will default to TCP Connect scans** to avoid requiring root, with an explicit `privileged: true` flag in workflow configs for SYN scans. This
is a design decision from §6 (Node Library Architecture).

### Step 2: Install Go (required for ProjectDiscovery tools)

subfinder, httpx, nuclei, and ffuf are all written in Go. Install Go first:

```bash
sudo pacman -S go
```

Verify:

```bash
go version
```

Make sure your Go bin path is in your `PATH`. Add to your `~/.zshrc` (or `~/.bashrc`):

```bash
echo 'export PATH="$PATH:$(go env GOPATH)/bin"' >> ~/.zshrc
source ~/.zshrc
```

### Step 3: Install ProjectDiscovery tools

```bash
# Subfinder — passive subdomain enumeration
go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest

# httpx — HTTP toolkit (probing, tech detection, status codes)
go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest

# nuclei — template-based vulnerability scanner
go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
```

> **Note:** These downloads compile from source and can take a few minutes each. The `-v` flag shows verbose output so you know it's working.

### Step 4: Install ffuf

```bash
go install github.com/ffuf/ffuf/v2@latest
```

### Step 5: Verify all tools

Run each one to confirm it's installed and in your PATH:

```bash
echo "=== Tool Verification ==="
echo ""

echo "nmap:"
nmap --version | head -1
echo ""

echo "subfinder:"
subfinder -version 2>&1 | head -1
echo ""

echo "httpx:"
httpx -version 2>&1 | head -1
echo ""

echo "nuclei:"
nuclei -version 2>&1 | head -1
echo ""

echo "ffuf:"
ffuf -version 2>&1 | head -1
echo ""

echo "=== All tools verified ==="
```

If any tool shows "command not found," check:
1. Is Go's bin path in your PATH? (`echo $PATH` — should contain `$HOME/go/bin`)
2. Did the installation complete without errors?
3. Open a new terminal and try again (shell config may need reloading)

### Step 6: Quick test of each tool

**nmap** — scan a legal target:

```bash
nmap -sT scanme.nmap.org -p 22,80,443 -oX /tmp/nmap_test.xml
```

The `-oX` flag outputs XML — this is the format Achilles will parse on Day 23. Take a look:

```bash
cat /tmp/nmap_test.xml
```

You'll see structured XML with host, port, state, and service data. This is what the nmap transformer will parse into ADC `Host` and `Port` objects.

**subfinder** — find subdomains:

```bash
subfinder -d hackerone.com -silent | head -10
```

This performs **passive** enumeration — it queries public sources (certificate transparency logs, DNS databases, search engines), not the 
target directly. Safe and legal for any domain.

**httpx** — probe discovered subdomains:

```bash
echo "hackerone.com" | httpx -silent -status-code -title
```

Shows the HTTP status code and page title. Achilles chains this after subfinder: discover subdomains → probe which ones are alive.

**nuclei** — update templates and scan:

```bash
# Download/update the template library (first run only — takes a minute)
nuclei -update-templates

# Scan with a specific safe template
echo "https://hackerone.com" | nuclei -t http/technologies/ -silent | head -5
```

> [!WARNING]
> nuclei templates can perform intrusive actions (SQLi testing, RCE attempts, etc.). **Only use safe categories** (`http/technologies/`, `http/miscellaneous/`)
on targets you don't own. On your own lab targets, anything goes.

**ffuf** — web fuzzing (test locally only):

```bash
# Just verify it runs — don't fuzz external targets without permission
ffuf -h | head -5
```

ffuf will be tested properly against your lab environment (Part 3). Clean up:

```bash
rm -f /tmp/nmap_test.xml
```

---

## Part 3: Set Up a Legal Target Environment

> [!CAUTION]
> **The rule, stated once, followed always:** Never scan targets you do not own or have explicit written authorization to scan. This is not a suggestion.
> It is the law (Computer Fraud and Abuse Act, Computer Misuse Act, IT Act 2000 in India, or your jurisdiction's equivalent). Violations carry criminal penalties.

You need dedicated, legal targets to develop and test Achilles against. Here are your options:

### Option A: Docker Vulnerable Lab (Recommended for Development)

Docker gives you instant, reproducible, isolated vulnerable targets. This is what you'll use most during Achilles development.

**Install Docker on Arch Linux:**

```bash
sudo pacman -S docker docker-compose

# Start and enable the Docker service
sudo systemctl enable --now 

# Add yourself to the docker group (avoids needing sudo for every docker command)
sudo usermod -aG docker $USER
```

> **Important:** Log out and log back in for the group change to take effect. Verify with `groups` — you should see `docker` listed.

**Launch DVWA (Damn Vulnerable Web Application):**

```bash
docker run -d --name dvwa -p 8080:80 vulnerables/web-dvwa
```

Open `http://localhost:8080` in your browser. Default credentials: `admin` / `password`. Click "Create / Reset Database" on first login.

**Now test your tools against it:**

```bash
# nmap — scan your local DVWA
nmap -sT localhost -p 8080

# httpx — probe it
echo "http://localhost:8080" | httpx -silent -status-code -title

# nuclei — scan for vulnerabilities (safe — it's YOUR container)
echo "http://localhost:8080" | nuclei -silent | head -20

# ffuf — fuzz for directories
ffuf -u http://localhost:8080/FUZZ -w /usr/share/seclists/Discovery/Web-Content/common.txt -mc 200,301,302 2>/dev/null | head -20
```

> **Wordlists for ffuf:** Install SecLists:
> ```bash
> # From AUR (if you use an AUR helper like yay or paru)
> yay -S seclists
>
> # Or clone directly
> git clone --depth 1 https://github.com/danielmiessler/SecLists.git ~/SecLists
> ```

**Stop and start DVWA as needed:**

```bash
docker stop dvwa        # Stop the container
docker start dvwa       # Start it again
docker rm -f dvwa       # Remove it entirely
```

### Option B: HackTheBox / TryHackMe (Recommended for Learning)

These platforms provide purpose-built vulnerable machines with guided walkthroughs.

- **[HackTheBox](https://hackthebox.com)** — Create an account. Start with "Starting Point" machines. Connect via OpenVPN: `sudo openvpn lab_phantom.ovpn` (they provide the config file). Each machine gets its own IP on the VPN — scan freely.

- **[TryHackMe](https://tryhackme.com)** — More guided, tutorial-style rooms. Great for learning specific attack techniques. Some rooms have in-browser attack boxes.

Install OpenVPN on Arch Linux for HTB:

```bash
sudo pacman -S openvpn
```

### Option C: KVM/QEMU Vulnerable VM Lab (Most Realistic)

For a full lab experience that simulates real pentesting — multiple services, real networking, actual OS — run a vulnerable VM locally with KVM/QEMU. This is the **native 
Linux hypervisor**, built into the kernel. It's faster than VirtualBox, more stable, and what professional pentesters use on Linux.

> **Why KVM/QEMU over VirtualBox or VMware?**
>
> | | KVM/QEMU | VirtualBox | VMware Workstation |
> |---|---|---|---|
> | **Performance** | Near-native (kernel module) | Good (kernel module) | Good (kernel module) |
> | **Linux integration** | Built into the kernel | Third-party | Third-party, proprietary |
> | **Arch Linux support** | First-class | Requires `virtualbox-host-modules-arch` | Requires AUR/manual install |
> | **License** | Free, open source | Free for personal use | Recently made free (was $250+) |
> | **Recommended for** | Linux users (you) | Cross-platform, GUI-first users | Users already invested in VMware |
>
> As an Arch Linux user, KVM/QEMU is the natural choice. It's in the official repos, maintained by the kernel team, and has no third-party kernel module conflicts.

---

#### Step C1: Verify hardware virtualization support

KVM requires hardware virtualization extensions (Intel VT-x or AMD-V). Check if your CPU supports it:

```bash
# Check for virtualization flags
grep -Ec '(vmx|svm)' /proc/cpuinfo
```

If the output is `0`, you need to enable virtualization in your BIOS/UEFI settings (usually under "CPU Configuration" or "Advanced"). If it's `1` or higher, you're good.

Also check if the KVM kernel modules are loaded:

```bash
lsmod | grep kvm
```

You should see `kvm` and either `kvm_intel` or `kvm_amd` depending on your CPU.

---

#### Step C2: Install KVM/QEMU and management tools

```bash
# Core virtualization stack
sudo pacman -S qemu-full libvirt virt-manager dnsmasq

# Breakdown:
#   qemu-full    — The actual emulator/hypervisor
#   libvirt      — VM management daemon (API layer over QEMU)
#   virt-manager — GUI for creating/managing VMs (uses libvirt)
#   dnsmasq      — DHCP/DNS for virtual networks
```

> **Optional but useful:**
> ```bash
> # If you want CLI-only management (no GUI)
> sudo pacman -S virt-install
>
> # For bridged networking (connecting VMs to your physical network)
> sudo pacman -S bridge-utils
>
> # UEFI firmware for VMs (if a VM needs UEFI boot)
> sudo pacman -S edk2-ovmf
> ```

---

#### Step C3: Enable and start the libvirt service

```bash
# Enable and start the libvirt daemon
sudo systemctl enable --now libvirtd.service

# Also enable the default virtual network
sudo virsh net-autostart default
sudo virsh net-start default
```

> If `net-start default` says the network is already active, that's fine.

---

#### Step C4: Add yourself to the libvirt group

By default, only root can manage VMs. Add your user to the `libvirt` group:

```bash
sudo usermod -aG libvirt $USER
```

**Log out and log back in** for the group change to take effect. Verify:

```bash
groups
```

You should see `libvirt` in the list.

Test that you can connect to the libvirt daemon:

```bash
virsh list --all
```

This should return an empty table (no VMs yet), not a permission error. If you get a connection error, try:

```bash
# Edit the libvirt config to allow your group
sudo nano /etc/libvirt/libvirtd.conf
```

Find and uncomment/set these lines:

```
unix_sock_group = "libvirt"
unix_sock_rw_perms = "0770"
```

Then restart the service:

```bash
sudo systemctl restart libvirtd
```

---

#### Step C5: Download a vulnerable VM image

**Metasploitable 2** is the classic practice target — an intentionally vulnerable Ubuntu VM with dozens of exploitable services (FTP, SSH, Telnet, HTTP, MySQL, PostgreSQL, Samba, and more).

```bash
# Create a directory for VM images
mkdir -p ~/VMs

# Download Metasploitable 2 (about 800MB)
cd ~/VMs
wget https://sourceforge.net/projects/metasploitable/files/Metasploitable2/metasploitable-linux-2.0.0.zip/download -O metasploitable2.zip

# Extract it
unzip metasploitable2.zip
```

This gives you a directory containing a `.vmdk` file (VMware disk format). QEMU can use it directly, but converting to its native format is faster:

```bash
# Convert VMDK to QCOW2 (QEMU's native format — supports snapshots, is more efficient)
qemu-img convert -f vmdk -O qcow2 \
    Metasploitable2-Linux/Metasploitable.vmdk \
    metasploitable2.qcow2

# Verify the image
qemu-img info metasploitable2.qcow2
```

You should see format `qcow2` and size around 2-3GB. You can now delete the original files:

```bash
rm -rf Metasploitable2-Linux metasploitable2.zip
```

> **Other vulnerable VMs to explore later:**
>
> | Image | Focus | Download |
> |-------|-------|----------|
> | **Metasploitable 2** | Multi-service Linux exploitation | sourceforge.net/projects/metasploitable |
> | **Metasploitable 3** | Windows/Linux, modernized services | github.com/rapid7/metasploitable3 |
> | **DVWA (VM)** | Web application vulnerabilities | vulnhub.com (search "DVWA") |
> | **Kioptrix** | Beginner-friendly Linux | vulnhub.com (search "Kioptrix") |
> | **Mr. Robot** | CTF-style, multi-step | vulnhub.com (search "Mr-Robot") |
>
> VulnHub ([vulnhub.com](https://vulnhub.com)) has hundreds of downloadable VMs organized by difficulty.

---

#### Step C6: Create an isolated network for your lab

You do NOT want your vulnerable VM on your home network. Create an **isolated host-only network** — your host (Arch) can reach the VM, but the VM has no internet access and no access to other devices on your network.

**Using virt-manager (GUI):**

1. Open `virt-manager` (search for "Virtual Machine Manager" in your app launcher, or run `virt-manager` from terminal)
2. Go to **Edit → Connection Details → Virtual Networks** tab
3. Click the **+** button (Add Network) at the bottom-left
4. Configure:
   - Name: `achilles-lab`
   - Mode: **Isolated** (no routing to physical network)
   - IPv4 network: `10.10.10.0/24`
   - Enable DHCP: Yes
   - DHCP range: `10.10.10.100` to `10.10.10.200`
5. Click **Finish**

**Using CLI (alternative):**

Create a network definition file:

```bash
cat > /tmp/achilles-lab-net.xml << 'EOF'
<network>
  <name>achilles-lab</name>
  <bridge name="virbr-lab"/>
  <ip address="10.10.10.1" netmask="255.255.255.0">
    <dhcp>
      <range start="10.10.10.100" end="10.10.10.200"/>
    </dhcp>
  </ip>
</network>
EOF

# Define, start, and auto-start the network
virsh net-define /tmp/achilles-lab-net.xml
virsh net-start achilles-lab
virsh net-autostart achilles-lab

# Verify
virsh net-list --all
```

You should see `achilles-lab` listed as `active` and `autostart: yes`.

**What this network does:**
- Your host machine is `10.10.10.1` — this is the gateway
- VMs get IPs in the `10.10.10.100-200` range via DHCP
- VMs can reach your host, and your host can reach VMs
- VMs **cannot** reach the internet or your LAN — fully isolated
- This prevents any accidental scanning of real external targets

---

#### Step C7: Create the Metasploitable 2 VM

**Using virt-manager (GUI):**

1. Open `virt-manager`
2. Click **File → New Virtual Machine** (or the "Create a new virtual machine" button)
3. Choose **"Import existing disk image"** → Forward
4. Browse to `~/VMs/metasploitable2.qcow2`
   - Click **Browse** → **Browse Local** → navigate to the file
   - OS type: select **"Generic Linux"** (or type "Ubuntu 8.04" — Metasploitable 2 is based on it)
5. Memory: **1024 MB** (1GB is plenty for Metasploitable 2)
6. CPUs: **1**
7. Name: `metasploitable2`
8. **Check "Customize configuration before install"** → Finish
9. In the configuration window:
   - Click **NIC** on the left panel
   - Change "Network source" from `default` to `achilles-lab` (your isolated network)
   - Change "Device model" to `virtio` (or leave as `e1000` — both work)
10. Click **Begin Installation**

**Using CLI (alternative):**

```bash
virt-install \
    --name metasploitable2 \
    --memory 1024 \
    --vcpus 1 \
    --disk path=$HOME/VMs/metasploitable2.qcow2,format=qcow2 \
    --import \
    --os-variant ubuntu8.04 \
    --network network=achilles-lab,model=virtio \
    --graphics spice \
    --noautoconsole
```

The VM should boot. Open its console in `virt-manager` (double-click the VM) or connect via:

```bash
virsh console metasploitable2
```

**Metasploitable 2 login credentials:**
- Username: `msfadmin`
- Password: `msfadmin`

---

#### Step C8: Find the VM's IP address

Once the VM is booted, find its IP:

**From inside the VM:**

```bash
# In the Metasploitable 2 console
ifconfig
# Look for the eth0 interface — IP should be in the 10.10.10.x range
```

**From your host (without logging into the VM):**

```bash
# List DHCP leases on the achilles-lab network
virsh net-dhcp-leases achilles-lab
```

This shows the VM's MAC address and assigned IP. It should be something like `10.10.10.100`.

**Verify connectivity from your host:**

```bash
ping -c 3 10.10.10.100    # Replace with actual IP
```

---

#### Step C9: Test all five tools against your VM

This is the payoff. Every tool, scanning a real vulnerable machine you own:

```bash
# Set the target IP (replace with your VM's actual IP)
export TARGET=10.10.10.100

echo "=== Scanning Metasploitable 2 at $TARGET ==="
echo ""
```

**nmap — full port scan:**

```bash
# TCP Connect scan — all common ports
nmap -sT $TARGET -p 1-10000 -oX /tmp/msf2_nmap.xml

# View the results
nmap -sT $TARGET -p 1-10000
```

You should see a LOT of open ports: 21 (FTP), 22 (SSH), 23 (Telnet), 25 (SMTP), 80 (HTTP), 139/445 (Samba), 3306 (MySQL), 5432 (PostgreSQL), and many more. This is intentional — Metasploitable 2 is designed to be vulnerable everywhere.

Look at the XML output:

```bash
cat /tmp/msf2_nmap.xml | head -50
```

This XML is what Achilles will parse on Day 23.

**httpx — probe HTTP services:**

```bash
echo "http://$TARGET" | httpx -silent -status-code -title -tech-detect
echo "http://$TARGET:8080" | httpx -silent -status-code -title -tech-detect
```

**nuclei — scan for vulnerabilities:**

```bash
# Full scan (safe — it's YOUR VM)
echo "http://$TARGET" | nuclei -silent | head -30
```

nuclei will likely find many issues — this is a VM designed to be vulnerable.

**ffuf — fuzz for web directories:**

```bash
# Fuzz the web server for directories
ffuf -u http://$TARGET/FUZZ \
    -w ~/SecLists/Discovery/Web-Content/common.txt \
    -mc 200,301,302 \
    -t 50
```

> If you haven't installed SecLists yet:
> ```bash
> git clone --depth 1 https://github.com/danielmiessler/SecLists.git ~/SecLists
> ```

Clean up:

```bash
rm -f /tmp/msf2_nmap.xml
```

---

#### Managing Your Lab VM

Commands you'll use regularly:

```bash
# Start the VM
virsh start metasploitable2

# Gracefully shut down the VM
virsh shutdown metasploitable2

# Force power off (if shutdown hangs — Metasploitable 2 sometimes does)
virsh destroy metasploitable2

# Check VM status
virsh list --all

# Take a snapshot (save the current state — restore if you break something)
virsh snapshot-create-as metasploitable2 clean-state "Fresh boot, no changes"

# List snapshots
virsh snapshot-list metasploitable2

# Restore a snapshot
virsh snapshot-revert metasploitable2 clean-state

# Delete the VM entirely (keeps the disk image)
virsh undefine metasploitable2

# Delete VM AND disk image
virsh undefine metasploitable2 --remove-all-storage
```

> [!TIP]
> **Take a snapshot immediately after first boot.** Metasploitable 2 is fragile — if you break something during testing, restoring the snapshot is instant. This mirrors real engagement practice: always snapshot before exploitation.

### Option D: VMware Workstation Pro (Alternative VM Option)

VMware Workstation Pro became **free for personal use** in late 2024 (after Broadcom acquired VMware). It's a mature, polished hypervisor with an excellent GUI. If you prefer VMware or already have experience with it, this is a perfectly valid choice.

> [!WARNING]
> **Arch Linux + VMware = maintenance overhead.** VMware's kernel modules (`vmmon`, `vmnet`) must be recompiled on every kernel update. On Arch's rolling release model, this happens frequently. If a module fails to build, your VMs won't start until it's fixed. KVM/QEMU (Option C) has zero maintenance since it's built into the kernel. Choose VMware only if you're comfortable with this trade-off.

---

#### Step D1: Install VMware Workstation Pro from AUR

VMware is not in the official Arch repos. You'll need an AUR helper (`yay` or `paru`):

```bash
# Install an AUR helper if you don't have one
sudo pacman -S --needed base-devel git
git clone https://aur.archlinux.org/yay.git /tmp/yay
cd /tmp/yay && makepkg -si
```

Now install VMware:

```bash
# This will download, compile kernel modules, and install VMware
yay -S vmware-workstation
```

> **This takes a while** — it downloads ~600MB and compiles kernel modules. If the build fails on kernel module compilation, check the AUR comments for patches. This is the trade-off mentioned above.

---

#### Step D2: Configure VMware system services

VMware needs two systemd services running:

```bash
# Load VMware kernel modules
sudo modprobe vmmon
sudo modprobe vmnet

# Enable the networking service (manages vmnet interfaces)
sudo systemctl enable --now vmware-networks.service

# Enable the USB arbitrator (if you need USB passthrough to VMs)
sudo systemctl enable --now vmware-usbarbitrator.service
```

Verify the modules are loaded:

```bash
lsmod | grep vm
```

You should see `vmmon` and `vmnet`.

---

#### Step D3: Handle kernel module signing (Secure Boot)

If you have **Secure Boot enabled**, VMware's unsigned kernel modules will be rejected. You have two options:

**Option 1: Disable Secure Boot** (simpler)
- Enter BIOS/UEFI → Security → Disable Secure Boot

**Option 2: Sign the modules yourself** (more secure)

```bash
# Create a signing key
sudo mkdir -p /etc/vmware/keys
cd /etc/vmware/keys
sudo openssl req -new -x509 -newkey rsa:2048 -keyout MOK.priv -outform DER -out MOK.der -nodes -days 36500 -subj "/CN=VMware Module Signing/"

# Enroll the key
sudo mokutil --import MOK.der
# You'll be asked to create a password — remember it

# Reboot — the MOK manager will appear, select "Enroll MOK" and enter your password
sudo reboot

# After reboot, sign the modules
sudo /usr/src/linux-headers-$(uname -r)/scripts/sign-file sha256 /etc/vmware/keys/MOK.priv /etc/vmware/keys/MOK.der $(modinfo -n vmmon)
sudo /usr/src/linux-headers-$(uname -r)/scripts/sign-file sha256 /etc/vmware/keys/MOK.priv /etc/vmware/keys/MOK.der $(modinfo -n vmnet)
```

> Most Arch users disable Secure Boot — it conflicts with custom kernels and various modules. If you're already running Arch without Secure Boot, skip this step entirely.

---

#### Step D4: Launch VMware and accept the license

```bash
vmware
```

On first launch:
1. Accept the license agreement (free for personal use)
2. Skip the license key prompt — leave it blank and click "Use VMware Workstation 17 for Free" (or equivalent)
3. VMware opens to the home screen

---

#### Step D5: Download Metasploitable 2

```bash
# Create a directory for VM images
mkdir -p ~/VMs

# Download Metasploitable 2
cd ~/VMs
wget https://sourceforge.net/projects/metasploitable/files/Metasploitable2/metasploitable-linux-2.0.0.zip/download -O metasploitable2.zip

# Extract it
unzip metasploitable2.zip
```

This gives you `Metasploitable2-Linux/` containing `.vmdk` and `.vmx` files. VMware can use these **directly** — no conversion needed (unlike KVM which requires VMDK → QCOW2).

---

#### Step D6: Create a host-only network (isolated lab)

You need an isolated network so the vulnerable VM can't reach the internet or your LAN.

1. Open VMware → **Edit → Virtual Network Editor**
   - If it asks for root, run: `sudo vmware-netcfg`
2. Click **Add Network** → select **vmnet2** (or any unused number)
3. Configure:
   - Type: **Host-only**
   - Subnet IP: `10.10.10.0`
   - Subnet mask: `255.255.255.0`
   - **Uncheck** "Connect a host virtual adapter to this network" if you want full isolation from your host's network stack (optional — leaving it checked is fine for scanning)
   - **Uncheck** "Use local DHCP service to distribute IP address to VMs" if you want to assign IPs manually, or **leave it checked** for automatic DHCP
4. Click **Apply** → **OK**

**What this creates:**
- A virtual switch (`vmnet2`) on the `10.10.10.0/24` subnet
- Your host gets `10.10.10.1` on this interface
- VMs on this network can talk to your host and each other
- VMs **cannot** reach the internet — fully isolated

---

#### Step D7: Import and configure the Metasploitable 2 VM

1. Open VMware → **File → Open** → navigate to `~/VMs/Metasploitable2-Linux/Metasploitable.vmx`
2. VMware may ask "Did you move or copy this VM?" — select **"I copied it"**
3. The VM appears in your library. **Before starting it**, configure the network:
   - Right-click the VM → **Settings**
   - Click **Network Adapter**
   - Change from "NAT" to **Custom: vmnet2** (your isolated host-only network)
   - Click **OK**
4. Adjust resources (optional):
   - Memory: **1024 MB** (1GB is plenty)
   - Processors: **1**
5. Click **Power on this virtual machine**

**Metasploitable 2 login credentials:**
- Username: `msfadmin`
- Password: `msfadmin`

---

#### Step D8: Find the VM's IP and verify connectivity

**From inside the VM:**

```bash
# In the Metasploitable 2 console
ifconfig
# Look for eth0 — IP should be in 10.10.10.x range
```

**From your host:**

```bash
# If DHCP is enabled, the VM gets an IP automatically
# Ping the subnet to discover it
nmap -sn 10.10.10.0/24
```

This will show your host (`10.10.10.1`) and the VM (e.g., `10.10.10.128`).

Verify:

```bash
ping -c 3 10.10.10.128    # Replace with actual IP
```

---

#### Step D9: Test all five tools against your VM

Same as the KVM/QEMU section — set the target and scan:

```bash
export TARGET=10.10.10.128    # Replace with your VM's IP

# nmap — full port scan
nmap -sT $TARGET -p 1-10000

# httpx — probe HTTP
echo "http://$TARGET" | httpx -silent -status-code -title -tech-detect

# nuclei — vulnerability scan (safe — it's YOUR VM)
echo "http://$TARGET" | nuclei -silent | head -30

# ffuf — directory fuzzing
ffuf -u http://$TARGET/FUZZ \
    -w ~/SecLists/Discovery/Web-Content/common.txt \
    -mc 200,301,302 \
    -t 50
```

---

#### Managing Your VMware Lab VM

**GUI:** Use the VMware Workstation interface — right-click the VM for power options, snapshots, settings.

**CLI (vmrun):**

```bash
# Start the VM (headless — no GUI window)
vmrun start ~/VMs/Metasploitable2-Linux/Metasploitable.vmx nogui

# Start with GUI
vmrun start ~/VMs/Metasploitable2-Linux/Metasploitable.vmx

# Stop the VM gracefully
vmrun stop ~/VMs/Metasploitable2-Linux/Metasploitable.vmx soft

# Force power off
vmrun stop ~/VMs/Metasploitable2-Linux/Metasploitable.vmx hard

# Take a snapshot
vmrun snapshot ~/VMs/Metasploitable2-Linux/Metasploitable.vmx "clean-state"

# Restore a snapshot
vmrun revertToSnapshot ~/VMs/Metasploitable2-Linux/Metasploitable.vmx "clean-state"

# List running VMs
vmrun list
```

---

#### Fixing VMware after Arch kernel updates

When Arch updates the kernel, VMware's modules will fail to load. Fix:

```bash
# Rebuild VMware kernel modules for the new kernel
sudo vmware-modconfig --console --install-all
```

If `vmware-modconfig` fails (common with very new kernels), check the AUR page for patches:

```bash
# Reinstall from AUR — this recompiles modules against the current kernel
yay -S vmware-workstation
```

> This is the primary maintenance cost of VMware on Arch. It usually takes 2-5 minutes but can be frustrating if patches are delayed. KVM/QEMU (Option C) never has this problem.

### Recommended Setup for Achilles Development

Use **Option A (Docker)** as your primary target during development — it's instant and disposable. Use **Option C (KVM/QEMU)** or **Option D (VMware)** when you need a full multi-service target for realistic tool testing. Use **Option B (HTB/THM)** for independent skill-building on the side.

---

## Part 4: Initialize the Real Achilles Project

Phase 0 used a practice repository. Now you create the real one.

### Step 1: Initialize Rust in the existing Achilles project

Your `~/Antigravity/Achilles/` directory already contains your docs, exercises, and roadmap. Initialize Cargo **inside it** — don't create a separate directory:

```bash
cd ~/Antigravity/Achilles
cargo init .
```

The `.` tells Cargo "initialize in the current directory." It adds two things without touching your existing files:
- `Cargo.toml` — Rust's package manifest (like `package.json` or `pom.xml`)
- `src/main.rs` — The entry point

> [!NOTE]
> Cargo may also create a `.gitignore` with `/target` in it. Since we'll replace it with our comprehensive version in Step 3, this is fine either way.

### Step 2: Verify it builds

```bash
cargo build
cargo run
```

You should see `Hello, world!`. This is the starting point of the Achilles engine.

### Step 3: Set up the enhanced .gitignore

Replace the default `.gitignore` with the comprehensive one from Day G3:

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
.vscode/                 # VS Code
*.swp                    # Vim swap files
*~                       # Emacs backup files
\#*\#                    # Emacs auto-save files
.dir-locals.el           # Emacs directory-local variables

# ─── OS Files ───────────────────────────────────────────
.DS_Store                # macOS Finder metadata
Thumbs.db                # Windows thumbnail cache
EOF
```

### Step 4: Initialize Git and make the first commit

```bash
git init
git add .
git commit -m "chore: initialize Achilles project with cargo"
```

### Step 5: Create the GitHub repository and push

1. Go to [github.com/new](https://github.com/new)
2. Repository name: `achilles` (if you renamed the practice repo) or a new name
3. Description: `Offensive Security Orchestration Engine — CLI-first security automation platform`
4. **Public**. No README, no .gitignore, no license (we'll add Apache 2.0 later).
5. Click **Create repository**

```bash
git remote add origin git@github.com:YOURUSERNAME/achilles.git
git push -u origin main
```

### Step 6: Verify the project structure

```bash
tree -a --gitignore
```

Expected:

```
.
├── .git/
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

This is the real Achilles. Every commit from now on is production code.

---

## Part 5: Key Concepts to Internalize

| Question | Your Answer Should Include |
|----------|--------------------------|
| Why use rustup over pacman's rust package? | Toolchain version management, component control, nightly/stable switching |
| What does `cargo-watch` do? | Auto-rebuilds on file change — `cargo watch -x run` |
| What does `cargo clippy` do? | Rust linter — catches mistakes and suggests idiomatic patterns |
| Why does nmap SYN scan need root? | SYN scan sends raw packets, which requires raw socket access (a Linux capability) |
| What's the difference between `-sT` and `-sS`? | `-sT` = TCP Connect (full handshake, no root). `-sS` = SYN (half-open, needs root, stealthier) |
| What is subfinder's scan type? | Passive — queries public databases, doesn't touch the target directly |
| What format does nmap XML output? | Structured XML with host, port, state, service data — what Achilles will parse |
| Why use Docker for lab targets? | Instant, reproducible, isolated, no risk of scanning external targets |
| What is the legal rule for scanning? | Never scan targets you don't own or have explicit written authorization to scan |
| What did `cargo init` create? | `Cargo.toml`, `src/main.rs`, `.gitignore` — a minimal compilable Rust binary project |

---

## Completion Checklist

Before moving to Phase 1, verify all of the following:

- [X] `rustc --version` shows a recent stable Rust (1.85+)
- [X] `cargo --version`, `cargo clippy --version`, `cargo fmt --version` all work
- [X] `cargo-watch` is installed (`cargo watch --version`)
- [X] `rust-analyzer` is installed and working in my editor
- [X] `nmap --version` works
- [X] `subfinder -version` works
- [X] `httpx -version` works
- [X] `nuclei -version` works
- [X] `ffuf -version` works
- [X] I have at least one legal target environment (Docker DVWA recommended)
- [X] I can scan my target with nmap and get XML output
- [X] The real Achilles project is initialized with `cargo init`, builds, and runs
- [X] The Achilles repo is on GitHub with the first commit
- [X] My `.gitignore` covers Rust, Achilles runtime, and editor files

---

## Lesson

> Professional pentesters set up lab environments before every engagement. Tool installation, target availability, and permission verification 
> happen before a single packet is sent. If you can't reproduce your target environment, you can't reproduce your results. Build this discipline now — it will define your career.

---

## Foundations Complete 🎉

Phase 0 and Phase 0b are done. You have:
- **Git mastery** — three-zone model, branching, rebasing, interactive history rewriting, conflict resolution, stash, reflog
- **A professional GitHub profile** — clean commits, branch workflow, PRs
- **Rust toolchain** — compiler, package manager, linter, formatter, language server
- **Five security tools** — installed, verified, tested against a legal target
- **The real Achilles project** — initialized, on GitHub, ready for code

**Next: Phase 1, Day 1 — Ownership, Borrowing, and the Borrow Checker →**
