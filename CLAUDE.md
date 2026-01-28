# Decision Graph - Claude Instructions

This project is a text-based knowledge graph for company-level decision making.

## Project Structure

```
src/              # Rust CLI source code
docs/             # Decision Graph data
  .decisions/     # Record files (DEC, STR, POL, CLI, OPP, PRC, HIR)
  .templates/     # Record templates
  .index.json     # Auto-generated index
.claude/          # Claude integration
  skills/         # Claude skills for decision management
  hooks/          # Post-chat hooks
```

## Using the CLI

Build and run:
```bash
cargo run -- <command>
```

Common commands:
```bash
dg init                          # Initialize
dg new decision "Title"          # Create record
dg list                          # List all
dg search "query"                # Search
dg show DEC-001                  # Show record
dg link DEC-001 depends_on STR-001  # Link records
dg graph                         # View graph
dg stats                         # Statistics
```

## When to Capture Decisions

Create records when:
- Business decisions are made
- Strategic direction is discussed
- Policies are established
- Client requirements are noted
- Market opportunities identified
- Processes defined
- Roles specified

## Record Types

| Type | Prefix | Framework |
|------|--------|-----------|
| Decision | DEC | SPADE |
| Strategy | STR | Six-Pager |
| Policy | POL | - |
| Client | CLI | - |
| Opportunity | OPP | OST |
| Process | PRC | DACI |
| Hiring | HIR | Scorecard |

## Link Types

- `supersedes` / `superseded_by` (auto-inverse)
- `depends_on`
- `enables`
- `relates_to`
- `conflicts_with`
- `refines`
- `implements`

## Development

- Run tests: `cargo test`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Avoiding Shells in Code

Per project policy, do not use shell commands from within the code. The CLI is the interface for shell operations.
