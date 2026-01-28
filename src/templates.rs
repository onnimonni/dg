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
client_info:
  industry: null
  size: null
  region: null
  tier: null
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

## Overview
<!-- Brief description of the client and relationship -->

## Business Context
<!-- Their industry, challenges, goals -->

### Industry Position

### Key Challenges

### Strategic Goals

## Our Engagement

### Products/Services Used
-

### Customizations Required
<!-- What special adaptations have we made for this client? -->

| Customization | Reason | Impact |
|--------------|--------|--------|
| | | |

## Key Contacts

| Name | Role | Notes |
|------|------|-------|
| | | |

## Requirements & Constraints

### Regulatory
<!-- Compliance requirements specific to this client -->

### Technical
<!-- Technical constraints or requirements -->

### Business
<!-- Business rules or constraints -->

## History

### Wins
<!-- Successful deliveries, positive feedback -->

### Issues
<!-- Problems encountered, lessons learned -->

### Requests
<!-- Pending or rejected feature requests -->

| Request | Status | Priority | Decision |
|---------|--------|----------|----------|
| | | | |
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
  headcount: 1
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

<!-- Scorecard Format (Who method - Geoff Smart & Randy Street) -->

## Mission
<!-- One sentence: Why does this role exist? -->

## Outcomes
<!-- 3-8 specific, measurable goals for year 1 -->

1. **Outcome**:
   **Measure**:
   **Timeline**:

2. **Outcome**:
   **Measure**:
   **Timeline**:

3. **Outcome**:
   **Measure**:
   **Timeline**:

## Competencies
<!-- Behaviors required to achieve outcomes -->

### Required Competencies

| Competency | Description | How We'll Assess |
|------------|-------------|------------------|
| | | |

### Nice-to-Have

| Competency | Description |
|------------|-------------|
| | |

## Technical Requirements
<!-- Hard skills, tools, certifications -->

### Must Have
-

### Preferred
-

## Cultural Fit
<!-- Values alignment, working style -->

-

## Interview Process

| Stage | Interviewer(s) | Focus | Duration |
|-------|---------------|-------|----------|
| Phone Screen | | Culture fit, basics | 30 min |
| Technical | | Technical competencies | 60 min |
| Hiring Manager | | Outcomes, motivation | 60 min |
| Team | | Collaboration | 45 min |

## Scorecard Template

<!-- For interviewers to use -->

| Criteria | 1 (Poor) | 2 | 3 (Meets) | 4 | 5 (Exceeds) | Score |
|----------|----------|---|-----------|---|-------------|-------|
| Outcome 1 | | | | | | |
| Outcome 2 | | | | | | |
| Competency 1 | | | | | | |
| Cultural Fit | | | | | | |

**Recommendation**: Strong No | No | Hire | Strong Hire

**Notes**:

## Compensation Range
<!-- If appropriate to document -->

- **Base**:
- **Bonus**:
- **Equity**:

## Timeline

| Milestone | Target Date |
|-----------|-------------|
| Job Posted | |
| Interviews Complete | |
| Offer Extended | |
| Start Date | |
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
    ]
}
