---
type: incident
id: INC-001
title: "Hooli Intellectual Property Lawsuit"
status: resolved
created: 2014-05-15
updated: 2014-08-20
authors: ["Richard Hendricks", "Ron LaFlamme"]
tags: [legal, hooli, ip, critical]
links:
  supersedes: []
  superseded_by: []
  depends_on: []
  enables: [DEC-004]
  relates_to: [ADR-001, DEC-001]
  conflicts_with: []
  refines: []
  implements: []
---

# Hooli Intellectual Property Lawsuit

## Summary

Hooli Inc. filed suit against Pied Piper claiming ownership of the Middle-Out compression algorithm, alleging Richard Hendricks developed it while employed at Hooli.

**Severity**: Critical (Existential Threat)
**Duration**: 3 months
**Resolution**: Victory via contract invalidation

## Timeline

| Date | Event |
|------|-------|
| 2014-05-15 | Hooli files IP lawsuit |
| 2014-05-16 | Series A funding frozen |
| 2014-06-01 | Discovery reveals Hooli laptop usage |
| 2014-07-15 | Binding arbitration begins |
| 2014-08-20 | Ruling in Pied Piper's favor |

## Root Cause Analysis

### Evidence Against Pied Piper

During discovery, it was revealed that Richard briefly tested a specific module of the compression code on a Hooli-issued laptop. Under standard IP assignment clauses in employment contracts, this would grant Hooli ownership.

### The Legal Technicality

Ron LaFlamme (legal counsel) identified that Hooli's employment contract contained a **Non-Compete Clause**. Under **California Business and Professions Code Section 16600**, non-compete agreements are unenforceable in California.

Because the contract contained an illegal clause, the entire employment agreement was voided—including the IP assignment clause.

## Resolution

The arbitrator ruled:
1. Richard DID breach his contract by using company resources
2. However, the contract itself was unenforceable due to the illegal non-compete
3. All IP rights revert to Richard Hendricks personally

## Impact

### Business Impact
- 3 months of operational paralysis
- Series A funding delayed
- Forced acceptance of "bad money" from Russ Hanneman (see DEC-004)

### Lessons Learned
1. Always have employment contracts reviewed by California-specialized counsel
2. Never use employer resources for personal projects, even briefly
3. Maintain clean room documentation for all core IP

## Follow-up Actions

- [x] Implement strict "clean room" development practices
- [x] Hire dedicated IP counsel
- [x] Document all algorithm development provenance
- [x] Create employee IP training program
