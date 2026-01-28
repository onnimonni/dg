# Decision Graph Format Specification

A text-based knowledge graph for company-level decision making.

## File Structure

All records use YAML frontmatter + Markdown body:

```markdown
---
type: decision|strategy|policy|customer|opportunity|process|hiring|adr|incident|runbook|meeting
id: DEC-001
title: Short descriptive title
status: draft|proposed|accepted|deprecated|superseded|active|open|closed|resolved|cancelled
created: 2024-01-15
updated: 2024-01-20
authors: [person1, person2]
tags: [market-expansion, europe, q1-2024]
links:
  supersedes: []
  superseded_by: []  # auto-added when another record supersedes this
  depends_on: [STR-001]
  enables: []
  relates_to: [CUS-001]
  conflicts_with: []
  refines: []
  implements: [STR-002]
---

# Title

## Context
What situation prompted this?

## Decision
What was decided?

## Consequences
What are the implications?
```

## Record Types

### Decision (DEC)
General business decisions using SPADE framework.

### Strategy (STR)
Strategic directions, market positioning (Six-Pager style).

### Policy (POL)
Internal policies, compliance requirements.

### Customer (CUS)
Customer-specific information, customizations, requirements.

### Opportunity (OPP)
Market opportunities using Opportunity Solution Tree format.

### Process (PRC)
Internal processes, workflows (DACI framework).

### Hiring (HIR)
Role definitions, scorecards, competencies.

### ADR (ADR)
Architecture Decision Records for technical decisions (Context, Decision, Consequences).

### Incident (INC)
Post-mortems and incident reports (blameless format, 5 Whys, timeline, action items).

### Runbook (RUN)
Step-by-step operational guides and how-tos (prerequisites, steps, verification, rollback).

### Meeting (MTG)
Meeting notes and minutes (agenda, discussion, decisions, action items).

## Link Types

| Type | Meaning |
|------|---------|
| `supersedes` | This record replaces another |
| `superseded_by` | This record was replaced (auto-added) |
| `depends_on` | Requires another record to be valid |
| `enables` | Makes another record possible |
| `relates_to` | General relationship |
| `conflicts_with` | Mutually exclusive |
| `refines` | More specific version of |
| `implements` | Concrete implementation of |

## Directory Structure

```
docs/
├── .decisions/
│   ├── DEC-001-expand-to-france.md
│   ├── STR-001-european-market-strategy.md
│   └── ...
├── .templates/
│   ├── decision.md
│   ├── strategy.md
│   └── ...
└── .index.json  # Auto-generated graph index
```

## ID Format

`{TYPE}-{NUMBER}[-{SLUG}]`

Examples:
- `DEC-001` (minimal)
- `DEC-042-expand-to-france` (with slug)
- `STR-003-q2-roadmap`
