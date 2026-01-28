# Decision Graph - Claude Instructions

This project uses `dg` to maintain a knowledge graph of decisions, architecture, processes, and operational knowledge. **You are expected to actively maintain this graph** as you work.

## Your Responsibilities

### 1. Check Context Before Acting

Before implementing features or making changes, search for related records:

```bash
dg search "<topic>"
dg list --type adr
```

If you find related records, **read them** and ensure your work aligns. If there's a conflict, **ask the user** before proceeding.

### 2. Capture Decisions As They Happen

When decisions are made during conversation, create records:

| Situation | Action |
|-----------|--------|
| Business decision made | `/decision` or `dg new decision` |
| Technical/architecture choice | `/adr` or `dg new adr` |
| Incident discussed | `/incident` or `dg new incident` |
| Process defined | `dg new process` |
| How-to explained | `/runbook` or `dg new runbook` |
| Meeting notes needed | `/meeting` or `dg new meeting` |

### 3. Ask for Clarification When

- A new decision **conflicts** with an existing record
- Something **seems familiar** but isn't documented
- User request **contradicts** existing architecture
- You're **unsure** if something was already decided

Example: "I found ADR-003 which says we use PostgreSQL. You're asking about MongoDB - should I update the ADR or is this a different use case?"

### 4. Link Records

Always connect new records to existing ones:

```bash
dg link ADR-XXX implements DEC-YYY
dg link INC-XXX relates_to ADR-ZZZ
dg link RUN-XXX enables PRC-WWW
```

## Record Types

| Type | Prefix | Use For |
|------|--------|---------|
| Decision | DEC | Business decisions (SPADE framework) |
| Strategy | STR | Strategic direction (Six-Pager) |
| Policy | POL | Internal policies, compliance |
| Customer | CUS | Architecture-impacting customer needs |
| Opportunity | OPP | Market opportunities (OST) |
| Process | PRC | Workflows, governance (DACI) |
| Hiring | HIR | Role definitions |
| ADR | ADR | Technical/architecture decisions |
| Incident | INC | Post-mortems, outages |
| Runbook | RUN | Operational how-tos |
| Meeting | MTG | Meeting notes |

## Quick Reference

```bash
# Search before acting
dg search "caching"

# Create records
dg new decision "Use vendor X for payments"
dg new adr "Adopt event sourcing"
dg new incident "API outage 2024-01-15"

# Link records
dg link ADR-001 implements DEC-005
dg link INC-002 enables RUN-003

# Update status
dg status DEC-001 accepted
dg status INC-001 resolved
dg status HIR-001 closed

# View context
dg show DEC-001
dg graph DEC-001
dg stats
```

## Link Types

| Link | Meaning |
|------|---------|
| `supersedes` | Replaces another record (auto-creates inverse) |
| `depends_on` | Requires another record |
| `enables` | Makes another record possible |
| `implements` | Concrete implementation of |
| `refines` | More specific version of |
| `relates_to` | General relationship |
| `conflicts_with` | Mutually exclusive |

## Project Structure

```
src/              # Rust CLI source
docs/
  .decisions/     # Record files
  .templates/     # Record templates
  .index.json     # Auto-generated index
.claude/
  skills/         # Slash command skills
  hooks/          # Session hooks
```

## Development

```bash
cargo test        # Run tests
cargo fmt         # Format code
cargo clippy      # Lint
cargo build --release  # Build CLI
```

## Asset Setup

Fonts are managed via Nix (inter, jetbrains-mono packages). They're copied automatically when entering devenv shell.

```bash
# Enter devenv shell (fonts copied automatically)
devenv shell

# Build CSS
css-build

# Or build everything (CSS + release binary)
build-all
```

Fonts used: Inter (UI), JetBrains Mono (code). Licensed under SIL OFL 1.1.

## Frontend Development

When working on `src/serve/templates.rs` or any HTML/CSS, follow the guidelines in `.claude/skills/frontend/skill.md`. Key points:

- Use existing design tokens (--bg, --surface, --text, etc.)
- Follow the visual hierarchy: Primary > Secondary > Tertiary
- Always add hover states and transitions for interactivity
- Consult Gemini via `mcp__consult-llm__consult_llm` for UX feedback
- Test visually with `dg serve` and Playwright screenshots

## Policy

- Do not use shell commands from within Rust code
- Capture decisions as they happen, not after
- Always link new records to existing context
- Ask when in doubt about conflicts
