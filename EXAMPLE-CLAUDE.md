# Decision Graph for Claude

This document guides Claude on how to use and maintain the Decision Graph - a text-based knowledge graph for company-level decision making.

## Overview

The Decision Graph (`dg` CLI) stores structured company knowledge in interconnected markdown files with YAML frontmatter. It captures:

- **Decisions (DEC)**: Business decisions using SPADE framework
- **Strategies (STR)**: Strategic directions using Six-Pager format
- **Policies (POL)**: Internal policies and compliance requirements
- **Customers (CUS)**: Customer-specific information and customizations
- **Opportunities (OPP)**: Market opportunities using OST format
- **Processes (PRC)**: Internal processes using DACI framework
- **Hiring (HIR)**: Role definitions and scorecards
- **ADRs (ADR)**: Architecture Decision Records for technical decisions

## When to Use Decision Graph

### Creating Records

Create records when the user:
- Makes a significant business decision that should be documented
- Discusses strategic direction or planning
- Mentions client-specific requirements or customizations
- Identifies a market opportunity or customer need
- Establishes or modifies an internal process
- Defines a new role or hiring requirement
- Establishes a policy or compliance requirement
- Makes a technical/architecture decision (use ADR)

### Searching Records

Search existing records when the user asks:
- "Why did we...?" - Search for relevant decisions
- "What's our policy on...?" - Search policies
- "What do we know about [customer]?" - Search customer records
- "How do we handle...?" - Search processes
- "Why did we choose [technology]?" - Search ADRs
- Questions about past decisions, company direction, or established practices

## CLI Usage

### Initialize (first time only)
```bash
dg init
```

### Create new record
```bash
dg new decision "Expand to France before Germany"
dg new strategy "European Market Entry 2024"
dg new customer "Acme Corp"
dg new policy "Data Retention Requirements"
dg new opportunity "Mobile App Feature Gap"
dg new process "Customer Onboarding"
dg new hiring "Senior Engineer"
dg new adr "Use PostgreSQL for Event Sourcing"
```

### Search
```bash
dg search "france"           # Search titles/tags
dg search -c "regulatory"    # Search content too
```

### List
```bash
dg list                      # All records
dg list -t decision          # Filter by type
dg list -s accepted          # Filter by status
dg list --tag europe         # Filter by tag
dg list -f json              # JSON output
```

### Show details
```bash
dg show DEC-001              # Show record
dg show DEC-001 -l           # Show with linked records
dg show DEC-001 --json       # JSON output
```

### Link records
```bash
dg link DEC-001 depends_on STR-001
dg link DEC-002 supersedes DEC-001
dg link DEC-001 relates_to CUS-001
```

Link types: `supersedes`, `depends_on`, `enables`, `relates_to`, `conflicts_with`, `refines`, `implements`

### Update status
```bash
dg status DEC-001 accepted
dg status DEC-001 deprecated
```

Statuses: `draft`, `proposed`, `accepted`, `deprecated`, `superseded`

### View graph
```bash
dg graph                     # Show all
dg graph DEC-001 -d 2        # From specific record, depth 2
dg graph -f dot              # Graphviz DOT format
dg graph -f json             # JSON format
```

### Other commands
```bash
dg stats                     # Show statistics
dg validate                  # Check for issues
dg export -f json            # Export all
dg reindex                   # Rebuild index
```

### Formatting & Linting
```bash
dg fmt                       # Format all markdown files
dg fmt --check               # Check formatting (for CI)
dg lint                      # Check for broken links
dg lint --strict             # Require tags and meaningful content
```

## File Format

Records are stored in `docs/decisions/` as markdown with YAML frontmatter:

```markdown
---
type: decision
id: DEC-001
title: "Expand to France before Germany"
status: accepted
created: 2024-01-15
updated: 2024-01-20
authors: [onni]
tags: [market-expansion, europe, q1-2024]
links:
  supersedes: []
  depends_on: [STR-001]
  enables: [OPP-003]
  relates_to: [CUS-002]
  conflicts_with: []
---

# Expand to France before Germany

## Setting
We need to decide which European market to enter first...

## People
- **Responsible**: CEO
- **Approvers**: Board
- **Consulted**: Sales, Legal
- **Informed**: All staff

## Alternatives

### Option A: France First
**Pros:**
- Larger market
- Existing contacts

**Cons:**
- Higher regulatory burden

### Option B: Germany First
...

## Decision
Chosen: **Option A: France First**

Rationale: Market size and existing relationships outweigh regulatory complexity.

## Consequences

### Positive
- Access to 67M population market

### Negative
- Need French legal entity
- 6-month regulatory approval process
```

## Best Practices

### For Claude

1. **Always search before creating** - Check if a similar record exists
2. **Link related records** - Connect decisions to strategies, customers, etc.
3. **Use appropriate types** - Match the record type to the content
4. **Add meaningful tags** - Enable discovery (e.g., `europe`, `q1-2024`, `regulatory`)
5. **Keep titles concise** - Searchable, descriptive titles
6. **Update status** - Mark records as accepted/deprecated as they evolve

### Suggested Tags

- Time: `q1-2024`, `2024`, `urgent`
- Region: `europe`, `france`, `germany`, `usa`
- Domain: `product`, `engineering`, `sales`, `legal`, `hr`
- Type: `regulatory`, `customer-request`, `internal`, `compliance`
- Priority: `critical`, `high`, `medium`, `low`

### When to Link

| From | To | Link Type |
|------|----|-----------|
| Decision | Strategy it implements | `implements` |
| Decision | Previous decision it replaces | `supersedes` |
| Decision | Strategy it depends on | `depends_on` |
| Decision | Customer it affects | `relates_to` |
| Opportunity | Decision that addresses it | `enables` |
| Policy | Regulation it complies with | `implements` |

## Example Workflow

User: "We decided to expand to France instead of Germany because of market size and our existing contacts there. This supports our European expansion strategy."

Claude actions:
1. Search for existing European strategy: `dg search "european expansion"`
2. Create decision: `dg new decision "Expand to France before Germany"`
3. Edit the created file to fill in details
4. Link to strategy: `dg link DEC-001 implements STR-001`
5. Add tags by editing file: `tags: [europe, france, market-expansion, q1-2024]`
6. Update status: `dg status DEC-001 accepted`

## ADR-Style Decision Chains

The Decision Graph excels at capturing evolving decisions - from strategy through obstacles to implementation. Use the `supersedes` link to show when plans change.

### Example Chain: Sweden Expansion

```
STR-002: Nordic Market Expansion (strategy)
    ↓ implements
DEC-002: Expand to Sweden (business decision)
    ↓ depends_on
DEC-003: Open SEK Bank Account with Nordea (requirement)
    ↓ supersedes (bank rejected application!)
DEC-004: Switch to Revolut for SEK Banking (pivot)
    ↓ implements
DEC-005: ADR - Integrate Revolut API (technical implementation)
```

### How Claude Should Handle This

**When initial plan is made:**
```bash
dg new decision "Open SEK Bank Account with Nordea"
dg link DEC-003 depends_on DEC-002
```

**When plan fails:**
```bash
# First, update the failed decision with UPDATE section explaining what happened
# Then create the new approach
dg new decision "Switch to Revolut for SEK Banking"
dg link DEC-004 supersedes DEC-003  # Automatically adds superseded_by inverse
```

**When technical implementation needed:**
```bash
dg new decision "ADR: Integrate Revolut API for Payments"
dg link DEC-005 implements DEC-004
# Add [adr, architecture] tags in the file
```

### Viewing the Chain
```bash
dg graph DEC-005 -d 4  # Shows full chain from ADR back to strategy
```

### Key Patterns

| Scenario | Link to Use |
|----------|-------------|
| Plan failed, trying alternative | New record `supersedes` old |
| Technical implementation of business decision | `implements` |
| This requires that first | `depends_on` |
| Enables future possibility | `enables` |
| General relationship | `relates_to` |

## Directory Structure

```
docs/
├── decisions/
│   ├── DEC-001-expand-to-france.md
│   ├── STR-001-european-market-strategy.md
│   ├── CUS-001-acme-corp.md
│   └── ...
├── .templates/
│   ├── decision.md
│   ├── strategy.md
│   └── ...
└── .index.json (auto-generated)
```
