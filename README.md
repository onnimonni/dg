# Decision Graph (dg)

A text-based knowledge graph for company-level decision making. Captures and interconnects decisions, strategies, policies, customers, opportunities, processes, hiring records, and architecture decision records (ADRs).

## Why?

Companies make thousands of decisions but rarely document them. When asked "Why did we expand to France instead of Germany?" or "What's our policy on data retention?", the answer is often "I think someone mentioned it in a meeting..."

Decision Graph solves this by:
- **Structured formats** based on proven frameworks (SPADE, Six-Pager, OST, DACI, Scorecards)
- **Interconnectivity** - decisions link to strategies, customers, policies
- **Text-based** - version controlled, searchable, Claude-friendly
- **CLI-first** - fast to create, query, and maintain

## Quick Start

```bash
# Install
cargo install --path .

# Initialize in your project
dg init

# Create your first decision
dg new decision "Use PostgreSQL for primary database"

# List all records
dg list

# Search
dg search "database"

# Show record with links
dg show DEC-001 -l

# View relationship graph
dg graph
```

## Record Types

| Type | Prefix | Framework | Use Case |
|------|--------|-----------|----------|
| Decision | DEC | SPADE | Business decisions with alternatives and rationale |
| Strategy | STR | Six-Pager | Strategic direction, market planning |
| Policy | POL | - | Internal policies, compliance requirements |
| Customer | CUS | - | Customer-specific info, customizations |
| Opportunity | OPP | OST | Market opportunities, customer needs |
| Process | PRC | DACI | Internal processes, workflows |
| Hiring | HIR | Scorecard | Role definitions, hiring criteria |
| ADR | ADR | ADR | Architecture decision records for technical decisions |

## Commands

### Creating Records

```bash
dg new decision "Title"
dg new strategy "Title"
dg new policy "Title"
dg new customer "Customer Name"
dg new opportunity "Title"
dg new process "Title"
dg new hiring "Role Title"
dg new adr "Title"
```

### Querying

```bash
# List all records
dg list
dg list -t decision          # Filter by type
dg list -s accepted          # Filter by status
dg list --tag europe         # Filter by tag
dg list -f json              # JSON output

# Search
dg search "query"            # Search titles/tags
dg search -c "query"         # Include content

# Show details
dg show DEC-001              # Show record
dg show DEC-001 -l           # With linked records
dg show DEC-001 --json       # JSON output
```

### Linking

```bash
# Create links
dg link DEC-001 depends_on STR-001
dg link DEC-002 supersedes DEC-001
dg link DEC-001 relates_to CUS-001

# Remove links
dg unlink DEC-001 relates_to CUS-001
```

**Link types:** `supersedes`, `depends_on`, `enables`, `relates_to`, `conflicts_with`, `refines`, `implements`

### Status Management

```bash
dg status DEC-001 accepted
dg status DEC-001 deprecated
```

**Statuses:** `draft`, `proposed`, `accepted`, `deprecated`, `superseded`

### Visualization

```bash
# Text view
dg graph                     # All records
dg graph DEC-001 -d 2        # From specific record, depth 2

# Graphviz DOT
dg graph -f dot > graph.dot
dot -Tpng graph.dot -o graph.png

# JSON
dg graph -f json
```

### Maintenance

```bash
dg stats                     # Show statistics
dg validate                  # Check for issues
dg export -f json            # Export all records
dg export -f csv -o out.csv  # Export to CSV
dg reindex                   # Rebuild index
```

### Formatting & Linting

```bash
dg fmt                       # Format all markdown files
dg fmt --check               # Check formatting (CI mode, exit 1 if unformatted)
dg fmt file1.md file2.md     # Format specific files

dg lint                      # Lint all records (check broken links)
dg lint --strict             # Strict mode (require tags, content)
dg lint --warn-orphans       # Warn about unlinked records
dg lint file1.md file2.md    # Lint specific files
```

## File Format

Records are markdown files with YAML frontmatter:

```markdown
---
type: decision
id: DEC-001
title: "Use PostgreSQL for primary database"
status: accepted
created: 2024-01-15
updated: 2024-01-20
authors: [alice, bob]
tags: [infrastructure, database, q1-2024]
links:
  supersedes: []
  depends_on: [STR-001]
  enables: [OPP-003]
  relates_to: [CUS-001]
  conflicts_with: []
---

# Use PostgreSQL for primary database

## Setting
...
```

## Directory Structure

```
docs/
├── .decisions/              # All record files
│   ├── DEC-001-use-postgresql.md
│   ├── STR-001-data-strategy.md
│   └── ...
├── .templates/              # Record templates
│   ├── decision.md
│   ├── strategy.md
│   └── ...
└── .index.json             # Auto-generated index
```

## Documentation

- [SUPPORTED-DOCS.md](SUPPORTED-DOCS.md) - Detailed documentation of all 8 record types, frameworks, and templates
- [EXAMPLE-CLAUDE.md](EXAMPLE-CLAUDE.md) - Guidance on using Decision Graph with Claude
- [docs/FORMAT.md](docs/FORMAT.md) - File format specification

### Skills

The `.claude/skills/decision-graph.md` provides Claude with:
- When to create records
- How to search and link
- Best practices for maintaining the knowledge graph

### Post-chat Hook

The `.claude/hooks/post-chat.md` reminds Claude to capture decisions made during conversations.

## Development

```bash
# Build
cargo build

# Test
cargo test

# Release build
cargo build --release

# Install locally
cargo install --path .
```

## Frameworks Referenced

- **SPADE** (Square): Setting, People, Alternatives, Decide, Explain
- **Six-Pager** (Amazon): Narrative memos for strategy
- **OST** (Teresa Torres): Opportunity Solution Trees
- **DACI** (Atlassian): Driver, Approver, Contributors, Informed
- **Scorecard** (Who method): Structured hiring
- **ADR** (Michael Nygard): Architecture Decision Records for technical decisions
