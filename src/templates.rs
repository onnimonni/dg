//! Embedded templates for record types

pub const DECISION_TEMPLATE: &str = r#"---
type: decision
id: DEC-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

## Setting
<!-- What is the context? Why now? What's at stake? -->

## People
<!-- SPADE Framework -->
- **Responsible**: <!-- Who owns the decision? -->
- **Approvers**: <!-- Who has veto power? -->
- **Consulted**: <!-- Who provides input? -->
- **Informed**: <!-- Who needs to know? -->

## Alternatives
<!-- List 3+ viable options with honest assessment -->

### Option A: {{NAME}}
**Pros:**
-

**Cons:**
-

### Option B: {{NAME}}
**Pros:**
-

**Cons:**
-

### Option C: {{NAME}}
**Pros:**
-

**Cons:**
-

## Decision
<!-- Which option was chosen and why? -->

Chosen: **Option X**

Rationale:

## Consequences

### Positive
-

### Negative
-

### Follow-up Actions
- [ ]
"#;

pub const STRATEGY_TEMPLATE: &str = r#"---
type: strategy
id: STR-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!-- Six-Pager Format (Amazon style) -->

## Introduction
<!-- Current state in 2-3 sentences. What's the situation? -->

## Goals
<!-- What specific outcomes are we targeting? Measurable where possible. -->

1.
2.
3.

## Tenets
<!-- Guiding principles for this strategy. What we will NOT compromise on. -->

1.
2.
3.

## State of the Business
<!-- Data-backed view of reality. Include metrics, trends, competitive landscape. -->

### Current Metrics

### Market Context

### Competitive Position

## Strategic Priorities
<!-- The actual plan. What will we do? In what order? -->

### Priority 1:

### Priority 2:

### Priority 3:

## Resource Requirements
<!-- What do we need to execute? -->

- **People**:
- **Budget**:
- **Timeline**:
- **Dependencies**:

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| | | | |

## FAQ
<!-- Anticipated questions and tough, honest answers -->

**Q: ?**
A:

## Success Criteria
<!-- How will we know this worked? -->

- [ ]
"#;

pub const POLICY_TEMPLATE: &str = r#"---
type: policy
id: POL-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
effective_date: {{DATE}}
review_date: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

## Purpose
<!-- Why does this policy exist? What problem does it solve? -->

## Scope
<!-- Who and what does this policy apply to? -->

- **Applies to**:
- **Does not apply to**:

## Background
<!-- What regulatory, legal, or business requirements drive this? -->

## Policy Statement
<!-- The actual policy. Clear, unambiguous language. -->

### Requirements

1.
2.
3.

### Prohibited Actions

1.
2.

### Exceptions
<!-- Under what conditions can this policy be bypassed? Who approves? -->

## Implementation

### Responsibilities

| Role | Responsibility |
|------|---------------|
| | |

### Procedures
<!-- Step-by-step implementation guidance -->

1.
2.
3.

## Compliance

### Monitoring
<!-- How will adherence be measured? -->

### Consequences
<!-- What happens if policy is violated? -->

## References
<!-- External regulations, laws, standards -->

-
"#;

pub const CUSTOMER_TEMPLATE: &str = r#"---
type: customer
id: CUS-{{NUMBER}}
title: "{{TITLE}}"
status: active
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!--
Customer Architecture Impact Record
NOTE: Only use this record type when a customer has requirements that
significantly impact our architecture, require custom features, or need
special handling. Do NOT store sensitive contact info here - use your CRM.
-->

## Overview
<!-- Brief description: Why does this customer need special architectural consideration? -->

## Architecture Impact

### Required Customizations
<!-- What architectural changes or customizations does this customer require? -->

| Customization | Architectural Impact | ADR Reference |
|--------------|---------------------|---------------|
| | | |

### Technical Constraints
<!-- Technical requirements that affect our system design -->

-

### Compliance Requirements
<!-- Regulatory/compliance needs that impact architecture (GDPR, SOC2, etc.) -->

-

## Integration Requirements

### APIs & Data Flows
<!-- Special integration needs -->

-

### Performance Requirements
<!-- SLAs, latency requirements, data volume -->

-

## Decisions Made

<!-- Link to relevant decisions made because of this customer's requirements -->

| Decision | Status | Link |
|----------|--------|------|
| | | |
"#;

pub const OPPORTUNITY_TEMPLATE: &str = r#"---
type: opportunity
id: OPP-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
opportunity_meta:
  outcome_metric: null
  target_value: null
  confidence: low
  effort: medium
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!-- Opportunity Solution Tree Format (Teresa Torres) -->

## Outcome
<!-- The business metric we want to move -->

**Target Metric**:
**Current Value**:
**Target Value**:
**Timeline**:

## Opportunity
<!-- The customer pain point or need -->

### Problem Statement

### Evidence
<!-- How do we know this is a real problem? -->

- **User Research**:
- **Data**:
- **Customer Feedback**:

### Affected Segments
<!-- Who experiences this problem? -->

## Solutions
<!-- Potential ways to address the opportunity -->

### Solution A: {{NAME}}

**Description**:

**Validation Status**: untested | assumption | validated

**Expected Impact**:

### Solution B: {{NAME}}

**Description**:

**Validation Status**: untested | assumption | validated

**Expected Impact**:

## Experiments
<!-- How we tested/will test solutions -->

| Experiment | Hypothesis | Result | Learning |
|------------|-----------|--------|----------|
| | | | |

## Decision
<!-- Which solution are we pursuing? Why? -->

**Chosen Solution**:

**Rationale**:

## Success Metrics
<!-- How will we measure if this worked? -->

| Metric | Baseline | Target | Actual |
|--------|----------|--------|--------|
| | | | |
"#;

pub const PROCESS_TEMPLATE: &str = r#"---
type: process
id: PRC-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
process_meta:
  frequency: as-needed
  duration: null
  last_review: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!-- DACI Framework -->

## Purpose
<!-- Why does this process exist? -->

## DACI

- **Driver**: <!-- Who herds the cats? -->
- **Approver**: <!-- Who makes the final call? -->
- **Contributors**: <!-- Who does the work? -->
- **Informed**: <!-- Who needs to know after? -->

## Trigger
<!-- What initiates this process? -->

## Inputs
<!-- What's needed to start? -->

-

## Process Steps

### Step 1: {{NAME}}
**Owner**:
**Duration**:

**Actions**:
1.
2.

**Output**:

### Step 2: {{NAME}}
**Owner**:
**Duration**:

**Actions**:
1.
2.

**Output**:

## Outputs
<!-- What's produced? -->

-

## Quality Checks
<!-- How do we know it was done correctly? -->

- [ ]

## Exceptions
<!-- What if something goes wrong? -->

| Scenario | Action | Escalation |
|----------|--------|------------|
| | | |

## Tools & Systems
<!-- What tools are used? -->

-

## Metrics
<!-- How do we measure process health? -->

| Metric | Target | Current |
|--------|--------|---------|
| | | |
"#;

pub const HIRING_TEMPLATE: &str = r#"---
type: hiring
id: HIR-{{NUMBER}}
title: "{{TITLE}}"
status: draft
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
hiring_meta:
  department: null
  level: null
  location: null
  reports_to: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!--
Role Definition Document
NOTE: Do NOT store candidate information here. Use your HRIS for candidate tracking.
This document defines the role requirements and evaluation criteria only.
-->

## Mission
<!-- One sentence: Why does this role exist? -->

## Outcomes
<!-- 3-5 specific, measurable goals for year 1 -->

1. **Outcome**:
   **Measure**:

2. **Outcome**:
   **Measure**:

3. **Outcome**:
   **Measure**:

## Required Competencies

| Competency | Description |
|------------|-------------|
| | |

## Technical Requirements

### Must Have
-

### Nice to Have
-

## Interview Focus Areas

| Stage | Focus |
|-------|-------|
| Screen | Culture fit, basics |
| Technical | Technical competencies |
| Hiring Manager | Outcomes, motivation |
| Team | Collaboration |

## Evaluation Criteria

| Criteria | What "Good" Looks Like |
|----------|------------------------|
| | |
"#;

pub const INCIDENT_TEMPLATE: &str = r#"---
type: incident
id: INC-{{NUMBER}}
title: "{{TITLE}}"
status: open
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
incident_meta:
  severity: sev1|sev2|sev3|sev4
  started: null
  detected: null
  resolved: null
  duration_minutes: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!-- Post-Mortem / Incident Report (Blameless format) -->

## Summary
<!-- 2-3 sentence summary of what happened -->

## Timeline

| Time | Event |
|------|-------|
| | Incident started |
| | Incident detected |
| | Response began |
| | Mitigated |
| | Resolved |

## Impact

- **Users affected**:
- **Duration**:
- **Revenue impact**:
- **Data loss**:

## Root Cause

<!-- What was the underlying cause? Use 5 Whys if helpful -->

## Contributing Factors

-

## What Went Well

-

## What Went Poorly

-

## Action Items

| Action | Owner | Due Date | Status |
|--------|-------|----------|--------|
| | | | |

## Lessons Learned

-

## Detection

<!-- How was this detected? How can we detect it faster? -->

## Prevention

<!-- What changes will prevent this from happening again? -->
"#;

pub const RUNBOOK_TEMPLATE: &str = r#"---
type: runbook
id: RUN-{{NUMBER}}
title: "{{TITLE}}"
status: accepted
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
runbook_meta:
  last_verified: null
  estimated_duration: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

<!-- Step-by-step operational guide -->

## Purpose
<!-- When and why would you use this runbook? -->

## Prerequisites

- [ ]

## Steps

### 1. {{STEP_NAME}}

```bash
# Commands or actions
```

**Expected outcome:**

### 2. {{STEP_NAME}}

```bash
# Commands or actions
```

**Expected outcome:**

### 3. {{STEP_NAME}}

```bash
# Commands or actions
```

**Expected outcome:**

## Verification

<!-- How do you know it worked? -->

- [ ]

## Rollback

<!-- If something goes wrong, how do you undo it? -->

## Troubleshooting

| Problem | Solution |
|---------|----------|
| | |

## Related

<!-- Links to related runbooks, docs, or decisions -->

-
"#;

pub const MEETING_TEMPLATE: &str = r#"---
type: meeting
id: MTG-{{NUMBER}}
title: "{{TITLE}}"
status: draft
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
meeting_meta:
  date: {{DATE}}
  attendees: []
  duration_minutes: null
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

## Attendees

-

## Agenda

1.
2.
3.

## Discussion

### Topic 1

**Summary:**

**Key points:**
-

### Topic 2

**Summary:**

**Key points:**
-

## Decisions Made

| Decision | Owner |
|----------|-------|
| | |

## Action Items

| Action | Owner | Due |
|--------|-------|-----|
| | | |

## Next Steps

-

## Follow-up Meeting

<!-- Date/time if scheduled -->
"#;

pub const ADR_TEMPLATE: &str = r#"---
type: adr
id: ADR-{{NUMBER}}
title: "{{TITLE}}"
status: proposed
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: [architecture]
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

## Context

<!-- What is the issue that we're seeing that motivates this decision or change? -->

## Decision

<!-- What is the change that we're proposing and/or doing? -->

## Consequences

### Positive

<!-- What becomes easier or possible as a result of this change? -->

-

### Negative

<!-- What becomes more difficult or impossible as a result of this change? -->

-

### Neutral

<!-- What other effects does this change have? -->

-

## Alternatives Considered

### Alternative 1: {{NAME}}

**Description:**

**Pros:**
-

**Cons:**
-

**Why not chosen:**

### Alternative 2: {{NAME}}

**Description:**

**Pros:**
-

**Cons:**
-

**Why not chosen:**

## References

<!-- Links to relevant documentation, RFCs, prior art, etc. -->

-

## Notes

<!-- Any additional information, implementation notes, or follow-up items -->

-
"#;

pub const LEGAL_TEMPLATE: &str = r#"---
type: legal
id: LEG-{{NUMBER}}
title: "{{TITLE}}"
status: draft
created: {{DATE}}
updated: {{DATE}}
authors: []
tags: []
legal_meta:
  legal_type: privacy  # privacy | tos | agreement | charter | other
  version: "1.0"
  effective_date: null
  parties: []
  jurisdiction: []
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: []
  relates_to: []
  conflicts_with: []
  refines: []
  implements: []
---

# {{TITLE}}

## Document Info

| Version | Effective | Status |
|---------|-----------|--------|
| 1.0 | TBD | Draft |

## Purpose
<!-- Why does this document exist? -->

## Scope
<!-- Who/what is covered -->

## Content
<!-- The actual policy/terms/agreement -->

## Related Records
<!-- Links to decisions that led to this -->

-
"#;

/// Get all templates as (filename, content) pairs
pub fn get_templates() -> Vec<(&'static str, &'static str)> {
    vec![
        ("decision.md", DECISION_TEMPLATE),
        ("strategy.md", STRATEGY_TEMPLATE),
        ("policy.md", POLICY_TEMPLATE),
        ("customer.md", CUSTOMER_TEMPLATE),
        ("opportunity.md", OPPORTUNITY_TEMPLATE),
        ("process.md", PROCESS_TEMPLATE),
        ("hiring.md", HIRING_TEMPLATE),
        ("adr.md", ADR_TEMPLATE),
        ("incident.md", INCIDENT_TEMPLATE),
        ("runbook.md", RUNBOOK_TEMPLATE),
        ("meeting.md", MEETING_TEMPLATE),
        ("legal.md", LEGAL_TEMPLATE),
    ]
}
