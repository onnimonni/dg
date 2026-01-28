---
type: process
id: PRC-001
title: Customer Onboarding
status: active
created: 2026-01-27
updated: 2026-01-28
authors: [jared, richard]
tags: [customer-success, onboarding, process, sales]
links:
  supersedes: []
  depends_on: []
  enables: []
  relates_to: [CUS-001, POL-001]
  conflicts_with: []
process_meta:
  frequency: as-needed
  duration: 2-4 weeks
  last_review: 2026-01-28
---

# Customer Onboarding

## Purpose

This process ensures new Pied Piper customers successfully adopt our middle-out compression platform. A structured onboarding reduces time-to-value, improves retention, and generates expansion opportunities. Jared's analytics show that customers onboarded within 14 days have 40% higher retention.

## DACI

- **Driver**: Customer Success Manager assigned to account
- **Approver**: Head of Customer Success (Jared)
- **Contributors**: Solutions Engineer, Support Engineer, Account Executive
- **Informed**: CEO (Richard), Sales Leadership, Product Team

## Trigger

- New customer contract signed
- Expansion deal closed with new use case
- Customer migration from legacy Hooli solution

## Inputs

- Signed contract with scope details
- Customer technical requirements document
- Key stakeholder contact information
- Integration requirements (API, SSO, etc.)
- Data residency requirements (see POL-001)

## Process Steps

### Step 1: Kickoff Preparation
**Owner**: Customer Success Manager
**Duration**: 1-2 business days

**Actions**:
1. Review contract scope and technical requirements
2. Schedule internal handoff meeting with Account Executive
3. Prepare kickoff deck customized for customer use case
4. Set up customer workspace in our systems
5. Assign Solutions Engineer based on technical complexity

**Output**: Kickoff meeting scheduled, internal team aligned

### Step 2: Customer Kickoff
**Owner**: Customer Success Manager + Solutions Engineer
**Duration**: 1 hour meeting + follow-ups

**Actions**:
1. Introductions and DACI alignment with customer
2. Review success criteria and timeline
3. Walk through middle-out compression architecture
4. Identify integration points and data sources
5. Schedule technical deep-dive sessions
6. Establish communication cadence (Slack, weekly calls)

**Output**: Kickoff notes, agreed timeline, technical assessment

### Step 3: Technical Implementation
**Owner**: Solutions Engineer
**Duration**: 1-2 weeks

**Actions**:
1. Configure customer environment (compression settings, storage)
2. Set up API credentials and webhooks
3. Integrate with customer's existing infrastructure
4. Configure SSO if enterprise tier (like Acme Corp, CUS-001)
5. Validate data residency compliance per POL-001
6. Run initial compression benchmarks - target 5.2x minimum

**Output**: Working technical integration, benchmark results

### Step 4: User Training
**Owner**: Customer Success Manager
**Duration**: 2-3 sessions over 1 week

**Actions**:
1. Admin training: configuration, user management, monitoring
2. Developer training: API usage, SDK integration, best practices
3. End-user training: dashboard, reports, alerts
4. Provide recorded sessions and documentation links
5. Set up customer sandbox for experimentation

**Output**: Trained users, training completion certificates

### Step 5: Go-Live and Handoff
**Owner**: Customer Success Manager
**Duration**: 1-3 business days

**Actions**:
1. Verify production readiness checklist complete
2. Migrate from pilot to production workloads
3. Monitor initial production usage closely
4. Address any issues immediately
5. Document lessons learned
6. Schedule 30-day check-in

**Output**: Customer live in production, handoff to ongoing support

## Outputs

- Production environment configured and validated
- Trained admin and end users
- Integration documentation specific to customer
- Baseline metrics (compression ratio, latency, throughput)
- 30/60/90 day success check-in schedule

## Quality Checks

- [ ] Compression ratio meets or exceeds 5.2x benchmark
- [ ] All key stakeholders have logged in at least once
- [ ] No critical support tickets in first 7 days
- [ ] Customer NPS score collected at go-live
- [ ] Technical documentation reviewed by customer

## Exceptions

| Scenario | Action | Escalation |
|----------|--------|------------|
| Customer delays kickoff >2 weeks | CSM follows up, escalate to AE | Head of CS after 3 weeks |
| Technical integration blocked | Solutions Engineering swarm | VP Engineering (Gilfoyle) |
| Compliance requirement not met | Pause deployment | Legal + Head of CS |
| Customer requests scope change | Document, assess impact | AE for commercial discussion |
| Go-live delayed >4 weeks | Red flag review meeting | CEO (Richard) if strategic |

## Tools & Systems

- Slack: Primary communication channel
- Zendesk: Support ticket management
- Notion: Customer workspace and documentation
- Datadog: Compression performance monitoring
- Salesforce: Account and opportunity tracking
- GitHub: API documentation and SDKs

## Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Time to first value | <14 days | 12 days |
| Onboarding NPS | >50 | 62 |
| 30-day active usage | >80% | 78% |
| Support tickets during onboarding | <3 | 2.1 |
| Onboarding completion rate | >95% | 94% |

## Revision History

| Date | Changes | Author |
|------|---------|--------|
| 2026-01-27 | Initial process definition | Jared |
| 2026-01-28 | Added metrics, refined steps | Richard |
