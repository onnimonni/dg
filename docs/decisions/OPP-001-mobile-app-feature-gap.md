---
type: opportunity
id: OPP-001
title: Mobile App Feature Gap
status: proposed
created: 2026-01-27
updated: 2026-01-27
authors: [dinesh, jared]
tags: [growth, mobile, product, user-experience]
links:
  relates_to: [CUS-001]
opportunity_meta:
  outcome_metric: mobile_dau
  target_value: 50000
  confidence: medium
  effort: high
---

# Mobile App Feature Gap

## Outcome

**Target Metric**: Mobile Daily Active Users (DAU)
**Current Value**: 12,000 DAU
**Target Value**: 50,000 DAU
**Timeline**: Q4 2024

## Opportunity

### Problem Statement

Enterprise users increasingly need to monitor their Pied Piper compression jobs from mobile devices. Our current mobile app only shows basic status and lacks the ability to configure, start, or troubleshoot compression jobs. Competitors like Hooli offer full mobile management capabilities.

### Evidence

- **User Research**: 67% of enterprise admins surveyed want mobile job management (n=234)
- **Data**: Mobile app sessions average 45 seconds vs 8 minutes on desktop, indicating users abandon due to limited functionality
- **Customer Feedback**: Acme Corp specifically requested mobile config capabilities in their enterprise contract renewal discussion

### Affected Segments

- Enterprise IT administrators managing large-scale compression deployments
- On-call engineers who need to respond to alerts outside office hours
- Field technicians at customer sites who need to troubleshoot remotely

## Solutions

### Solution A: Native Mobile Job Management

**Description**: Build full job management into native iOS and Android apps, including job creation, configuration editing, and real-time monitoring with push notifications.

**Validation Status**: assumption

**Expected Impact**: High - addresses core user need directly. Would match Hooli's feature set.

### Solution B: Responsive Web Dashboard

**Description**: Rebuild web dashboard to be fully responsive on mobile browsers, avoiding native app development overhead.

**Validation Status**: validated (prototype tested with 5 users)

**Expected Impact**: Medium - faster to ship but worse UX than native. Users prefer native apps for frequent tasks.

### Solution C: Mobile API + Third-Party Integration

**Description**: Expose comprehensive API that allows users to build their own mobile workflows via Shortcuts (iOS) or Tasker (Android).

**Validation Status**: untested

**Expected Impact**: Low - appeals to power users only. Most enterprises want out-of-box solutions.

## Experiments

| Experiment | Hypothesis | Result | Learning |
|------------|-----------|--------|----------|
| Prototype test (n=5) | Responsive web is sufficient | Failed | Users struggled with touch targets, wanted native feel |
| Feature flag API preview | Power users will build integrations | Partial success | 3/10 users tried, all wanted simpler option |
| Competitor analysis | Hooli mobile capabilities | Confirmed | Hooli has full native job management |

## Decision

**Chosen Solution**: Solution A (Native Mobile Job Management)

**Rationale**: Prototype testing showed responsive web doesn't meet user expectations. Competitive pressure from Hooli makes parity a priority. Native apps will also enable push notifications for alert management, a frequently requested feature.

## Success Metrics

| Metric | Baseline | Target | Actual |
|--------|----------|--------|--------|
| Mobile DAU | 12,000 | 50,000 | - |
| Mobile session duration | 45 sec | 5 min | - |
| Mobile job starts | 0 | 1,000/day | - |
| App Store rating | 2.8 | 4.5 | - |
