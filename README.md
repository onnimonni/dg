# Decision Graph (dg)

A text-based knowledge graph for company-level decision making. Captures and interconnects decisions, strategies, policies, architecture decisions, incidents, runbooks, and more.

## Why?

Companies make thousands of decisions but rarely document them. When asked "Why did we expand to France?" or "What's our caching strategy?", the answer is often lost in Slack threads or forgotten meetings.

Decision Graph solves this by:
- **Structured formats** - proven frameworks (SPADE, Six-Pager, ADR, DACI)
- **Interconnectivity** - decisions link to strategies, incidents, runbooks
- **Text-based** - version controlled, searchable, AI-friendly
- **Claude integration** - automatic decision capture during conversations

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
    flake: false
```

```nix
# devenv.nix
{ inputs, ... }:
{
  imports = [ "${inputs.dg}/devenv-module.nix" ];
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
| Strategy | STR | Strategic direction (Six-Pager) |
| Policy | POL | Internal policies, compliance |
| Customer | CUS | Architecture-impacting customer needs |
| Opportunity | OPP | Market opportunities (OST) |
| Process | PRC | Workflows, governance (DACI) |
| Hiring | HIR | Role definitions |
| ADR | ADR | Architecture decisions |
| Incident | INC | Post-mortems, outages |
| Runbook | RUN | Operational how-tos |
| Meeting | MTG | Meeting notes |

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
├── .decisions/     # Record files
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

- **SPADE** (Square): Setting, People, Alternatives, Decide, Explain
- **Six-Pager** (Amazon): Narrative memos for strategy
- **ADR** (Michael Nygard): Architecture Decision Records
- **DACI** (Atlassian): Driver, Approver, Contributors, Informed
- **OST** (Teresa Torres): Opportunity Solution Trees
- **Scorecard** (Who method): Structured hiring
- **5 Whys**: Root cause analysis for incidents

## License

MIT
