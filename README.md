# Decision Graph (dg)

**The knowledge graph that lives with your code.** Capture decisions, strategies, and institutional knowledge in markdown files that version control alongside your codebase.

> *"Why did we choose PostgreSQL?" "Who decided on the France expansion?" "What's our caching strategy?"*
>
> Stop losing decisions in Slack threads and forgotten meetings.

## For Humans and AI

Decision Graph is built for the async-first, AI-augmented workplace:

- **Colocated with code** - decisions live in git, reviewed in PRs, deployed with releases
- **AI-native** - structured formats that LLMs can read, query, and update
- **Framework-driven** - proven templates (SPADE, Six-Pager, 7 Powers, RICE)
- **Graph-connected** - decisions link to strategies, incidents, customers, feedback

### Key Features

| Feature | Description |
|---------|-------------|
| **Multi-player mode** | Draft IDs prevent conflicts, GitHub Actions auto-finalize |
| **User & Team management** | @mentions, profiles, team hierarchies |
| **Strategic frameworks** | 7 Powers analysis, RICE scoring, OST |
| **Web UI** | Browse, search, edit with live preview |
| **Claude Code integration** | Auto-capture decisions during coding sessions |

## Installation

### With Nix/Devenv (Recommended)

Initialize a new project with Decision Graph:

```bash
nix flake init -t github:onnimonni/dg
devenv shell
```

Or add to existing devenv project:

```yaml
# devenv.yaml
inputs:
  dg:
    url: github:onnimonni/dg
```

```nix
# devenv.nix
{ pkgs, inputs, ... }:
let system = pkgs.stdenv.hostPlatform.system;
in {
  imports = [ inputs.dg.devenvModules.default ];
  packages = [ inputs.dg.packages.${system}.default ];
}
```

### With Cargo

```bash
cargo install --git https://github.com/onnimonni/dg
```

### From Releases

Download pre-built binaries from [Releases](https://github.com/onnimonni/dg/releases):
- `dg-linux-x86_64` - Linux x86_64
- `dg-linux-aarch64` - Linux ARM64
- `dg-macos-x86_64` - macOS Intel
- `dg-macos-aarch64` - macOS Apple Silicon
- `dg-windows-x86_64.exe` - Windows

## Quick Start

```bash
# Initialize in your project
dg init

# Create records
dg new decision "Use PostgreSQL for primary database"
dg new adr "Adopt event sourcing for order service"
dg new incident "API outage 2024-01-15"

# Link records
dg link ADR-001 implements DEC-001

# Query
dg list
dg search "database"
dg show DEC-001

# Visualize
dg graph
```

## Record Types

| Type | Prefix | Use Case |
|------|--------|----------|
| Decision | DEC | Business decisions (SPADE framework) |
| Strategy | STR | Strategic direction (Six-Pager + 7 Powers) |
| Policy | POL | Internal policies, compliance |
| Customer | CUS | Architecture-impacting customer needs |
| Opportunity | OPP | Market opportunities (OST + RICE scoring) |
| Process | PRC | Workflows, governance (DACI) |
| Hiring | HIR | Role definitions |
| ADR | ADR | Architecture decisions |
| Incident | INC | Post-mortems, outages |
| Runbook | RUN | Operational how-tos |
| Meeting | MTG | Meeting notes |
| Feedback | FBK | Customer feedback and feature requests |

## Commands

```bash
# Create
dg new <type> "Title"

# Query
dg list [--type TYPE] [--status STATUS] [--tag TAG]
dg search "query" [-c]  # -c includes content
dg show ID [-l]         # -l shows linked records

# Link
dg link ID1 <link_type> ID2
dg unlink ID1 <link_type> ID2

# Status
dg status ID <status>

# Visualize
dg graph [ID] [-d DEPTH] [-f dot|json|text]

# Validate
dg lint [--strict] [--warn-orphans]
dg fmt [--check]

# Maintain
dg stats
dg validate
dg reindex
dg export [-f json|csv]
```

### Link Types

| Link | Meaning |
|------|---------|
| `supersedes` | Replaces another (auto-creates inverse) |
| `depends_on` | Requires another record |
| `enables` | Makes another possible |
| `implements` | Concrete implementation of |
| `refines` | More specific version of |
| `relates_to` | General relationship |
| `conflicts_with` | Mutually exclusive |

### Statuses

`draft` → `proposed` → `accepted` → `deprecated` / `superseded`

Additional: `active`, `open`, `closed`, `resolved`, `cancelled`

## Claude Code Integration

When using with devenv, Claude automatically:
- Searches for related records before making changes
- Captures decisions during conversations
- Links new records to existing context
- Asks for clarification when conflicts found

### Skills

Use slash commands to create records:
- `/decision` - business decision
- `/adr` - architecture decision
- `/incident` - post-mortem
- `/runbook` - operational guide
- `/meeting` - meeting notes
- `/context` - search before acting

### Hooks

- **Session start**: Reminds about decision graph context
- **Session stop**: Prompts to capture uncaptured decisions

## File Format

Records are markdown with YAML frontmatter:

```markdown
---
type: decision
id: DEC-001
title: "Use PostgreSQL for primary database"
status: accepted
created: 2024-01-15
updated: 2024-01-20
authors: [alice]
tags: [database, infrastructure]
links:
  implements: [STR-001]
  enables: [ADR-002]
---

# Use PostgreSQL for primary database

## Setting
...
```

## Directory Structure

```
docs/
├── decisions/      # Record files
├── .templates/     # Record templates
└── .index.json     # Auto-generated index
.claude/
├── hooks/          # Session hooks
└── skills/         # Slash commands
```

## Development

```bash
cargo build           # Build
cargo test            # Test
cargo build --release # Release build
cargo install --path . # Install locally
```

## Frameworks

Decision Graph includes templates for proven decision-making frameworks:

- **SPADE** (Square): Setting, People, Alternatives, Decide, Explain
- **Six-Pager** (Amazon): Narrative memos for strategy
- **7 Powers** (Hamilton Helmer): Competitive moat analysis
- **RICE** (Intercom): Reach, Impact, Confidence, Effort prioritization
- **ADR** (Michael Nygard): Architecture Decision Records
- **DACI** (Atlassian): Driver, Approver, Contributors, Informed
- **OST** (Teresa Torres): Opportunity Solution Trees
- **Scorecard** (Who method): Structured hiring
- **5 Whys**: Root cause analysis for incidents

## License

MIT
