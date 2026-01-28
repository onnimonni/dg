# Supported Document Types

`dg` is a cli tool to manage company information in monorepo.

Decision Graph supports 8 record types, each with a specific prefix, framework, and use case.

## Record Types Overview

| Type | Prefix | Framework | Status Options | Use Case |
|------|--------|-----------|----------------|----------|
| [Decision](#decision-dec) | DEC | SPADE | draft, proposed, accepted, deprecated, superseded | Business decisions with alternatives and rationale |
| [Strategy](#strategy-str) | STR | Six-Pager | draft, proposed, accepted, deprecated, superseded | Strategic direction, market planning |
| [Policy](#policy-pol) | POL | - | draft, proposed, accepted, deprecated, superseded | Internal policies, compliance requirements |
| [Customer](#customer-cus) | CUS | - | active, deprecated | Customer-specific info, customizations |
| [Opportunity](#opportunity-opp) | OPP | OST | draft, proposed, accepted, deprecated | Market opportunities, customer needs |
| [Process](#process-prc) | PRC | DACI | draft, proposed, accepted, deprecated | Internal processes, workflows |
| [Hiring](#hiring-hir) | HIR | Scorecard | draft, open, filled, cancelled | Role definitions, hiring criteria |
| [ADR](#adr-adr) | ADR | ADR | proposed, accepted, deprecated, superseded | Architecture decision records |

## Link Types

Records can be connected using 8 relationship types:

| Link Type | Inverse | Description |
|-----------|---------|-------------|
| `supersedes` | `superseded_by` | This record replaces another (auto-creates inverse) |
| `superseded_by` | `supersedes` | This record was replaced by another |
| `depends_on` | - | Requires another record to be valid |
| `enables` | - | Makes another record possible |
| `relates_to` | - | General relationship |
| `conflicts_with` | - | Mutually exclusive with another |
| `refines` | - | More specific version of another |
| `implements` | - | Concrete implementation of another |

---

## Decision (DEC)

**Framework:** SPADE (Square) - Setting, People, Alternatives, Decide, Explain

**Use for:** Business decisions that need documented rationale, alternatives considered, and consequences tracked.

**Template structure:**
```markdown
# {Title}

## Setting
Context, why now, what's at stake

## People
- **Responsible**: Who owns the decision
- **Approvers**: Who has veto power
- **Consulted**: Who provides input
- **Informed**: Who needs to know

## Alternatives
### Option A: {Name}
**Pros:** ...
**Cons:** ...

### Option B: {Name}
**Pros:** ...
**Cons:** ...

## Decision
Chosen: **Option X**
Rationale: ...

## Consequences
### Positive
### Negative
### Follow-up Actions
```

**Example:**
```bash
dg new decision "Expand to France before Germany"
```

---

## Strategy (STR)

**Framework:** Six-Pager (Amazon) - Narrative memo format for strategic thinking

**Use for:** Strategic direction, market positioning, long-term planning, roadmaps.

**Template structure:**
```markdown
# {Title}

## Introduction
Current state in 2-3 sentences

## Goals
1. Specific, measurable outcomes
2. ...

## Tenets
Guiding principles we won't compromise

## State of the Business
### Current Metrics
### Market Context
### Competitive Position

## Strategic Priorities
### Priority 1: ...
### Priority 2: ...

## Resource Requirements
- People, Budget, Timeline, Dependencies

## Risks and Mitigations
| Risk | Likelihood | Impact | Mitigation |

## FAQ
Anticipated questions with honest answers

## Success Criteria
How we'll measure success
```

**Example:**
```bash
dg new strategy "European Market Expansion 2024"
```

---

## Policy (POL)

**Framework:** Standard policy format

**Use for:** Internal policies, compliance requirements, regulatory mandates, company rules.

**Template structure:**
```markdown
# {Title}

## Purpose
Why this policy exists

## Scope
- **Applies to**: ...
- **Does not apply to**: ...

## Background
Regulatory, legal, or business drivers

## Policy Statement
### Requirements
### Prohibited Actions
### Exceptions

## Implementation
### Responsibilities
| Role | Responsibility |

### Procedures
Step-by-step guidance

## Compliance
### Monitoring
### Consequences

## References
External regulations, laws, standards
```

**Example:**
```bash
dg new policy "Data Retention Requirements"
```

---

## Customer (CUS)

**Framework:** Customer profile template

**Use for:** Customer-specific information, customizations, requirements, relationship history.

**Template structure:**
```markdown
# {Customer Name}

## Overview
Brief description of client and relationship

## Business Context
### Industry Position
### Key Challenges
### Strategic Goals

## Our Engagement
### Products/Services Used
### Customizations Required
| Customization | Reason | Impact |

## Key Contacts
| Name | Role | Notes |

## Requirements & Constraints
### Regulatory
### Technical
### Business

## History
### Wins
### Issues
### Requests
| Request | Status | Priority | Decision |
```

**Example:**
```bash
dg new customer "Acme Corp"
```

---

## Opportunity (OPP)

**Framework:** OST (Teresa Torres) - Opportunity Solution Trees

**Use for:** Market opportunities, customer pain points, feature ideas, product discovery.

**Template structure:**
```markdown
# {Title}

## Outcome
**Target Metric**: ...
**Current Value**: ...
**Target Value**: ...
**Timeline**: ...

## Opportunity
### Problem Statement
### Evidence
- User Research
- Data
- Customer Feedback

### Affected Segments

## Solutions
### Solution A: {Name}
**Description**: ...
**Validation Status**: untested | assumption | validated
**Expected Impact**: ...

## Experiments
| Experiment | Hypothesis | Result | Learning |

## Decision
**Chosen Solution**: ...
**Rationale**: ...

## Success Metrics
| Metric | Baseline | Target | Actual |
```

**Example:**
```bash
dg new opportunity "Mobile App Feature Gap"
```

---

## Process (PRC)

**Framework:** DACI (Atlassian) - Driver, Approver, Contributors, Informed

**Use for:** Internal processes, workflows, standard operating procedures.

**Template structure:**
```markdown
# {Title}

## Purpose
Why this process exists

## DACI
- **Driver**: Who herds the cats
- **Approver**: Who makes the final call
- **Contributors**: Who does the work
- **Informed**: Who needs to know after

## Trigger
What initiates this process

## Inputs
What's needed to start

## Process Steps
### Step 1: {Name}
**Owner**: ...
**Duration**: ...
**Actions**: ...
**Output**: ...

## Outputs
What's produced

## Quality Checks
How we know it was done correctly

## Exceptions
| Scenario | Action | Escalation |

## Tools & Systems

## Metrics
| Metric | Target | Current |
```

**Example:**
```bash
dg new process "Customer Onboarding"
```

---

## Hiring (HIR)

**Framework:** Scorecard (Who method - Geoff Smart & Randy Street)

**Use for:** Role definitions, job scorecards, hiring criteria, interview processes.

**Template structure:**
```markdown
# {Role Title}

## Mission
One sentence: Why does this role exist?

## Outcomes
1. **Outcome**: ...
   **Measure**: ...
   **Timeline**: ...

## Competencies
### Required Competencies
| Competency | Description | How We'll Assess |

### Nice-to-Have
| Competency | Description |

## Technical Requirements
### Must Have
### Preferred

## Cultural Fit
Values alignment, working style

## Interview Process
| Stage | Interviewer(s) | Focus | Duration |

## Scorecard Template
| Criteria | 1 (Poor) | 2 | 3 (Meets) | 4 | 5 (Exceeds) | Score |

## Compensation Range
- Base, Bonus, Equity

## Timeline
| Milestone | Target Date |
```

**Example:**

```bash
dg new hiring "Senior Engineer"
```

---

## ADR (ADR)

**Framework:** ADR (Michael Nygard) - Architecture Decision Records

**Use for:** Technical/architecture decisions, technology choices, system design decisions.

**Template structure:**
```markdown
# {Title}

## Status
Proposed | Accepted | Deprecated | Superseded

## Context
What is the issue that motivates this decision?

## Decision
What is the change we're proposing/doing?

## Consequences
### Positive
What becomes easier or possible?

### Negative
What becomes more difficult?

### Neutral
Other effects

## Alternatives Considered
### Alternative 1: {Name}
**Description**: ...
**Pros**: ...
**Cons**: ...
**Why not chosen**: ...

## References
Links to docs, RFCs, prior art

## Notes
Implementation notes, follow-up items
```

**Example:**
```bash
dg new adr "Use PostgreSQL for Event Sourcing"
```

---

## File Format

All records use YAML frontmatter + Markdown body:

```yaml
---
type: decision          # Record type (lowercase)
id: DEC-001             # Unique identifier
title: "Title Here"     # Display title
status: proposed        # Current status
created: 2024-01-15     # Creation date
updated: 2024-01-20     # Last modified
authors: [alice, bob]   # Contributors
tags: [tag1, tag2]      # Searchable tags
links:
  supersedes: []        # Records this replaces
  depends_on: []        # Prerequisites
  enables: []           # What this makes possible
  relates_to: []        # Related records
  conflicts_with: []    # Mutually exclusive
---

# Title

Markdown content here...
```

---

## CLI Quick Reference

```bash
# Create records
dg new decision "Title"
dg new strategy "Title"
dg new policy "Title"
dg new customer "Name"
dg new opportunity "Title"
dg new process "Title"
dg new hiring "Role"
dg new adr "Title"

# Query
dg list                      # All records
dg list -t decision          # Filter by type
dg list -s accepted          # Filter by status
dg search "query"            # Search titles/tags
dg show DEC-001              # View record
dg show DEC-001 -l           # With linked records

# Link management
dg link DEC-001 depends_on STR-001
dg link DEC-002 supersedes DEC-001
dg unlink DEC-001 relates_to CUS-001

# Maintenance
dg validate                  # Check for issues
dg lint                      # Lint records
dg lint --strict             # Strict validation
dg fmt                       # Format markdown
dg fmt --check               # Check formatting
dg stats                     # Show statistics
dg graph                     # View relationships
dg graph -f dot              # Graphviz output
```
