---
type: strategy
id: STR-001
title: Pivot from B2C Music App to B2B Infrastructure
status: accepted
created: 2014-05-20
updated: 2014-06-15
authors: [richard, erlich]
tags:
- b2b
- compression
- pivot
- strategy
links:
  enables: [ADR-002, ADR-003, ADR-004]
foundational: false
---

# Pivot from B2C Music App to B2B Infrastructure

## Introduction

Pied Piper was originally conceived as a consumer music application that could identify songs and detect copyright infringement. However, during development, Richard Hendricks accidentally created a revolutionary compression algorithm that achieves unprecedented Weissman scores. After winning TechCrunch Disrupt, we must decide whether to continue as a consumer app or pivot to enterprise infrastructure.

## Goals

1. Achieve $10M ARR within 24 months
2. Establish Pied Piper as the industry standard for data compression
3. Build defensible technology moat against Hooli and other competitors
4. Create sustainable business model that attracts Series A funding

## Tenets

1. **Algorithm integrity**: We will not compromise the quality of middle-out compression for short-term gains
2. **Technical excellence**: Engineering decisions will be made by engineers, not business people
3. **Independence**: We will not be acquired by Hooli or compromise our values for funding

## State of the Business

### Current Metrics

- Users (music app): 12,000 beta testers
- Compression ratio: 5.2 Weissman score (industry-leading)
- Team size: 5 engineers
- Runway: 8 months at current burn rate
- Revenue: $0

### Market Context

The global data compression market is valued at $4.5B and growing 12% annually. Enterprise data storage costs are exploding, with companies spending billions on infrastructure. Cloud providers and media companies are desperate for better compression solutions.

### Competitive Position

- **Hooli/Nucleus**: Pursuing similar technology but bureaucratic and slow
- **EndFrame**: Inferior compression ratios, focusing on video only
- **Traditional codecs**: H.264, H.265 are widely adopted but approaching theoretical limits

## Strategic Priorities

### Priority 1: Enterprise API Platform

Build a cloud-based API that allows enterprises to compress data using middle-out algorithm. Target initial customers in media, healthcare, and financial services where data volumes are highest.

### Priority 2: Strategic Partnerships

Partner with cloud providers (AWS, Azure, GCP) to offer integrated compression services. Explore OEM licensing deals with storage vendors.

### Priority 3: Patent Protection

File comprehensive patent portfolio around middle-out compression to create defensive moat and licensing opportunities.

## Resource Requirements

- **People**: 3 additional backend engineers, 1 sales/BD hire
- **Budget**: $2M Series A funding (seeking from Raviga Capital)
- **Timeline**: 6 months to MVP, 12 months to GA
- **Dependencies**: Legal support for patent filings, cloud infrastructure

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Hooli copies technology | High | Critical | Accelerate patent filings, maintain speed advantage |
| Enterprise sales cycles too long | Medium | High | Offer freemium tier, target mid-market first |
| Algorithm has undiscovered flaws | Low | Critical | Extensive testing, bug bounty program |
| Team conflict on direction | Medium | Medium | Clear decision rights, regular alignment meetings |

## FAQ

**Q: Why abandon the music app after all the work?**
A: The music app was always a means to an end. The compression technology has 100x the market opportunity.

**Q: What about our existing beta users?**
A: We'll sunset the music app gracefully with 90-day notice. Some may become enterprise leads.

**Q: Can we compete with Hooli's resources?**
A: Speed and focus beat resources. Hooli has thousands of employees; we have five people who actually understand the algorithm.

## Success Criteria

- [ ] First paying enterprise customer within 6 months
- [ ] $1M ARR within 12 months
- [ ] Weissman score maintained above 5.0 at scale
- [ ] Series A closed at $10M+ valuation
