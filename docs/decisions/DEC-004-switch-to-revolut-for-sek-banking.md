---
type: decision
id: DEC-004
title: Switch to Revolut for SEK Banking
status: accepted
created: 2026-01-27
updated: 2026-01-27
authors: [finance-team]
tags: [banking, fintech, revolut, sek, sweden]
links:
  supersedes: [DEC-003]
---

# Switch to Revolut for SEK Banking

## Setting

Nordea rejected our bank account application (DEC-003). We need an alternative solution to receive SEK payments and operate in Sweden. This is blocking our Sweden launch (DEC-002).

## People

- **Responsible**: CFO
- **Approvers**: CEO
- **Consulted**: Engineering (for API integration)
- **Informed**: Finance team, Sales

## Alternatives

### Option A: Revolut Business
**Pros:**
- Setup in 24-48 hours
- SEK IBAN available
- API for payment automation
- Lower transaction fees

**Cons:**
- Not a traditional Swedish bank
- Some enterprise customers may question legitimacy

### Option B: Wise Business
**Pros:**
- Similar fast setup
- Good multi-currency support

**Cons:**
- No Swedish IBAN (uses UK routing)
- Some Swedish customers can't pay to non-SE IBANs

### Option C: Try Another Traditional Bank
**Pros:**
- Traditional banking relationship

**Cons:**
- Likely same rejection issues
- Another 4-6 week delay

## Decision

Chosen: **Option A: Revolut Business**

Rationale: Speed is critical. Revolut provides legitimate Swedish IBAN. Engineering team can integrate their API for automated reconciliation.

## Consequences

### Positive
- Unblocked within 48 hours
- API integration enables automation
- Lower ongoing costs

### Negative
- Some perception risk with traditional enterprises

### Follow-up Actions
- [x] Open Revolut Business account
- [ ] Integrate Revolut API (see DEC-005)
- [ ] Update customer payment instructions
