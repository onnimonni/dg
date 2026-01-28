---
type: process
id: PRC-001
title: OEM Licensing Program
status: active
created: 1981-08-12
updated: 2001-11-02
authors: [billg, steveb]
tags: [business-model, distribution, licensing, oem]
links:
  supersedes: []
  superseded_by: []
  depends_on: [POL-001, DEC-006]
  enables: [STR-001]
  relates_to: [CUS-001]
  conflicts_with: []
  refines: []
  implements: []
core: true
---

# OEM Licensing Program

## Purpose

The OEM (Original Equipment Manufacturer) Licensing Program is Microsoft's primary distribution mechanism for operating systems and bundled software. It enables PC manufacturers to pre-install Windows on every machine they ship.

## Process Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Microsoft   │───▶│ OEM Partner │───▶│ End Customer│
│ (Software)  │    │ (Hardware)  │    │ (PC Buyer)  │
└─────────────┘    └─────────────┘    └─────────────┘
     │                   │
     ▼                   ▼
  Per-Copy           Windows
  Royalty           Pre-installed
```

## Key Terms

### Standard OEM Agreement
1. **Per-copy royalty**: Fixed fee per unit shipped (not per unit sold)
2. **Volume discounts**: Lower rates for higher volumes
3. **Marketing development funds**: Co-marketing dollars for promoting Windows
4. **Technical support allocation**: Shared support responsibilities
5. **Logo certification**: Requirements for "Designed for Windows" branding

### Historical Rate Structure (Confidential)

| Era | Approximate DOS/Windows Rate |
|-----|------------------------------|
| 1981-1985 | $10-15 per copy |
| 1985-1990 | $15-25 per copy |
| 1990-1995 | $25-50 per copy |
| 1995-2000 | $50-75 per copy |

*Rates varied by volume and relationship*

## Strategic Genius of the Model

### 1. Guaranteed Revenue
PC makers pay royalties on units **shipped**, not sold. Returns, inventory, and channel issues are the OEM's problem, not Microsoft's.

### 2. Zero Distribution Cost
Microsoft doesn't manufacture, ship, or stock physical product. OEMs handle all logistics.

### 3. Network Effects
Every OEM pre-install means:
- Another Windows user
- Another reason for developers to target Windows
- Higher switching costs for the OEM itself

### 4. Commoditizing Hardware
By licensing non-exclusively, Microsoft ensured OEMs competed on hardware price and features while Microsoft captured software margin. This is the inverse of Apple's integrated model.

## Process Steps

### For New OEM Partners

1. **Application** — OEM requests partnership
2. **Technical Qualification** — Hardware compatibility testing
3. **Commercial Negotiation** — Volume commitments, pricing
4. **Contract Execution** — License agreement signed
5. **Technical Integration** — OEM builds Windows into manufacturing
6. **Logo Certification** — Testing for "Designed for Windows" compliance
7. **Ongoing Compliance** — Quarterly audits and reporting

### For Existing Partners

1. **Quarterly Forecasting** — OEM provides unit projections
2. **Royalty Payment** — Monthly or quarterly remittance
3. **Audit Rights** — Microsoft may verify unit counts
4. **Version Upgrades** — Transition to new Windows versions

## Antitrust Modifications (Post-2001)

The consent decree (INC-001) modified OEM agreements:

1. **No exclusivity** — OEMs may preinstall competitor software
2. **Desktop flexibility** — OEMs may customize Windows desktop
3. **Uninstall rights** — Users must be able to remove IE
4. **No retaliation** — Cannot penalize OEMs for Linux offerings

## Metrics

| Metric | 1990 | 2000 | 2010 |
|--------|------|------|------|
| OEM Partners | ~50 | ~100 | ~200 |
| Units Licensed | ~30M | ~200M | ~350M |
| OEM Revenue | ~$1B | ~$10B | ~$15B |

## Roles & Responsibilities

| Role | Responsibility |
|------|----------------|
| **OEM Account Managers** | Relationship management, contract negotiation |
| **Technical Evangelists** | Driver support, compatibility testing |
| **Finance** | Royalty collection, audit enforcement |
| **Legal** | Contract compliance, antitrust adherence |
| **Marketing** | Co-marketing programs, logo certification |

## Success Factors

The OEM model worked because:
1. IBM PC standard created compatible hardware ecosystem
2. DOS/Windows backward compatibility protected OEM investments
3. Microsoft's "Switzerland" position (licensing to all) prevented OEM revolt
4. Volume economics made Windows cheaper than alternatives

This process is the operational execution of STR-001's "success reinforces success" vision.
