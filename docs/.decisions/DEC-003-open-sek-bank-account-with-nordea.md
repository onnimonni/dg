---
type: decision
id: DEC-003
title: Open SEK Bank Account with Nordea
status: superseded
created: 2026-01-27
updated: 2026-01-27
authors: [finance-team]
tags: [banking, nordea, sek, sweden]
links:
  superseded_by: [DEC-004]
  depends_on: [DEC-002]
---

# Open SEK Bank Account with Nordea

## Setting

To operate in Sweden (DEC-002), we need a local SEK bank account for:
- Receiving payments from Swedish customers
- Paying Swedish suppliers and contractors
- Complying with local tax requirements

Nordea is the largest Nordic bank with strong presence in Sweden.

## People

- **Responsible**: CFO
- **Approvers**: CEO
- **Consulted**: Legal, Tax advisor
- **Informed**: Finance team

## Alternatives

### Option A: Nordea (Traditional Bank)
**Pros:**
- Established Swedish bank
- Full banking services
- Local branch network

**Cons:**
- Lengthy KYC process (4-6 weeks)
- High monthly fees
- Requires physical presence for setup

### Option B: SEB
**Pros:**
- Another major Swedish bank
- Similar services to Nordea

**Cons:**
- Same lengthy process
- Less favorable rates for foreign companies

### Option C: Fintech (Revolut/Wise)
**Pros:**
- Fast setup (days not weeks)
- Lower fees
- API access for automation

**Cons:**
- Not a "real" Swedish bank
- Some suppliers may not accept

## Decision

Chosen: **Option A: Nordea**

Rationale: Traditional bank provides credibility with Swedish enterprise customers.

## Consequences

### Positive
- Professional appearance for Swedish market
- Full banking relationship potential

### Negative
- 4-6 week setup delay

### Follow-up Actions
- [x] Submit KYC documentation
- [x] Await bank approval

---

## UPDATE: Application Rejected

**Date**: 2026-01-20

Bank denied our application due to:
- Insufficient operating history in EU
- Risk profile of our industry

**Next Steps**: See DEC-004 for alternative approach
