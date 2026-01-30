---
type: process
id: PRC-002
title: Enterprise Agreement Model
status: active
created: 1995-01-15
updated: 2020-01-01
authors: [steveb]
tags: [b2b, enterprise, licensing, recurring-revenue]
links:
  supersedes: []
  superseded_by: []
  depends_on: [STR-002]
  enables: []
  relates_to: [DEC-008]
  conflicts_with: []
  refines: []
  implements: []
---

# Enterprise Agreement Model

## Purpose

The Enterprise Agreement (EA) is Microsoft's licensing model for large organizations. It transforms software from a capital expenditure (one-time purchase) into an operational expenditure (annual subscription), creating predictable recurring revenue.

## The Problem Solved

### Before Enterprise Agreements

1. **Upgrade pricing collapse** — Once a company bought Office 95, why buy Office 97?
2. **Administrative chaos** — IT departments tracked individual licenses per seat
3. **Piracy risk** — Employees installed software on unauthorized machines
4. **Unpredictable revenue** — Boom/bust around major version releases

### Steve Ballmer's Innovation

Bundle everything. Charge per machine per year. Simplify procurement. Increase switching costs.

## Agreement Structure

### Standard Enterprise Agreement

```
┌────────────────────────────────────────────────────┐
│              3-Year Enterprise Agreement            │
├────────────────────────────────────────────────────┤
│ Year 1: Full payment (or 1/3 upfront)              │
│ Year 2: True-up for added machines                 │
│ Year 3: True-up + renewal negotiation              │
└────────────────────────────────────────────────────┘
```

### Components

| Component | Includes |
|-----------|----------|
| **Desktop Platform** | Windows, Office, CALs |
| **Server Platform** | Windows Server, SQL Server, Exchange, SharePoint |
| **Enterprise Services** | Premier Support, Training |
| **Cloud Services** | Microsoft 365, Azure credits |

### Pricing Tiers

| Tier | Seats | Discount |
|------|-------|----------|
| EA Standard | 250-2,499 | Baseline |
| EA Subscription | 250-2,499 | Monthly option |
| Enterprise Enrollment | 2,500-9,999 | Additional discount |
| Enterprise Enrollment + | 10,000+ | Maximum discount |

## Process Flow

### Initial Sale

1. **Account Planning** — Enterprise sales rep identifies opportunity
2. **Needs Assessment** — Current license audit, growth projections
3. **Proposal** — Custom bundle based on infrastructure
4. **Negotiation** — Pricing, terms, deployment timeline
5. **Legal Review** — Contract finalization
6. **Signature** — Executive approval both sides
7. **Deployment** — Software delivery and activation

### Annual Cycle

```
Q1: Anniversary planning
Q2: True-up reporting (count added machines)
Q3: True-up payment
Q4: Renewal negotiation begins
```

## Strategic Benefits

### For Microsoft

1. **Predictable revenue** — Multi-year commitments smooth quarterly results
2. **Higher lifetime value** — Bundling increases total spend
3. **Reduced churn** — 3-year contracts lock in customers
4. **Upsell opportunities** — Annual reviews surface new needs

### For Customers

1. **Simplified procurement** — One agreement covers everything
2. **Budget predictability** — Known annual expense
3. **Latest versions** — Automatic upgrade rights
4. **Volume discounts** — Better pricing than retail

## Lock-in Mechanics

The EA creates multiple layers of switching cost:

1. **Financial** — Prepaid multi-year commitments
2. **Technical** — Deep integration across Microsoft stack
3. **Process** — IT workflows built around Microsoft tools
4. **Training** — Workforce skilled on Microsoft products
5. **Data** — Documents, emails, databases in Microsoft formats

## Metrics

| Metric | 1995 | 2005 | 2015 |
|--------|------|------|------|
| EA Customers | ~500 | ~15,000 | ~50,000 |
| EA Revenue | ~$500M | ~$15B | ~$30B |
| Avg Deal Size | ~$1M | ~$1M | ~$600K |
| Renewal Rate | — | ~90% | ~95% |

## Evolution to Microsoft 365

The EA model evolved into Microsoft 365 subscriptions:

| Era | Model | Key Change |
|-----|-------|------------|
| 1995-2010 | Classic EA | Software licenses |
| 2010-2015 | EA + Cloud | Hybrid licenses |
| 2015-present | Microsoft 365 | Cloud-first subscriptions |

The subscription model extends EA principles to smaller organizations, creating "EA-like" recurring revenue at scale.

## Roles & Responsibilities

| Role | Responsibility |
|------|----------------|
| **Enterprise Account Executive** | Relationship owner, deal negotiation |
| **Technical Account Manager** | Deployment support, architecture |
| **Licensing Specialist** | Agreement compliance, true-up |
| **Customer Success Manager** | Adoption, satisfaction, renewal |

This process is the operational heart of STR-002's enterprise flywheel strategy.
