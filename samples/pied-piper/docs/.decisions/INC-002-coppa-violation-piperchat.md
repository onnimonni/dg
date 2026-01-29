---
type: incident
id: INC-002
title: "COPPA Violation PiperChat"
status: resolved
created: 2016-04-25
updated: 2016-05-10
authors: [dinesh, gilfoyle]
tags:
- children
- compliance
- legal
- piperchat
incident_meta:
  severity: sev1
  started: 2016-04-20
  detected: 2016-04-25
  resolved: 2016-05-10
  duration_minutes: 28800
links:
  enables: [POL-001]
---

# COPPA Violation PiperChat

## Summary

PiperChat, our video chat application, was found to be collecting personal information from users under 13 years old without parental consent, violating the Children's Online Privacy Protection Act (COPPA). An estimated 23,000 users were potentially under 13, exposing Pied Piper to significant FTC fines and legal liability.

## Timeline

| Time | Event |
|------|-------|
| 2016-04-20 | FTC inquiry letter received by legal |
| 2016-04-25 | Engineering team notified, investigation begins |
| 2016-04-26 | Confirmed age verification absent from signup |
| 2016-04-28 | Emergency age gate deployed |
| 2016-05-01 | Underage accounts identified and suspended |
| 2016-05-10 | Settlement agreement reached with FTC |

## Impact

- **Users affected**: ~23,000 potentially underage users
- **Duration**: Unknown period since launch (estimated 4 months)
- **Revenue impact**: $0 direct (PiperChat was free), but legal costs ~$500K
- **Data loss**: Personal data collected from minors had to be deleted

## Root Cause

PiperChat was launched without any age verification mechanism. The signup flow only required email and phone number. No one on the team considered COPPA compliance during product development because we assumed our users would be adults.

**5 Whys Analysis:**
1. Why were minors able to sign up? No age verification existed.
2. Why was there no age verification? It wasn't in the product requirements.
3. Why wasn't it in the requirements? No legal review of the product spec.
4. Why was there no legal review? We didn't have legal counsel at the time.
5. Why didn't we have legal counsel? We prioritized engineering speed over compliance.

## Contributing Factors

- Lack of legal expertise on staff
- "Move fast and break things" culture without compliance guardrails
- PiperChat was a skunkworks project built during a coding competition
- No product management oversight

## What Went Well

- Fast response once the issue was identified
- Transparent communication with FTC
- Team rallied to implement fixes quickly
- No data breach or malicious use of minor data

## What Went Poorly

- Took 5 days from FTC letter to engineering notification
- Initial denial of problem severity by leadership
- No existing processes for compliance incidents
- Media coverage damaged company reputation

## Action Items

| Action | Owner | Due Date | Status |
|--------|-------|----------|--------|
| Implement age gate on all products | Dinesh | 2016-05-01 | Complete |
| Delete all data from underage users | Gilfoyle | 2016-05-05 | Complete |
| Hire compliance officer | Richard | 2016-06-01 | Complete |
| Create compliance review process | Jared | 2016-06-15 | Complete |
| Establish Tethics framework | Richard | 2016-07-01 | Complete |

## Lessons Learned

- Legal compliance cannot be an afterthought
- Every user-facing product needs legal review before launch
- Age verification should be a standard component
- "Move fast" doesn't mean "ignore regulations"

## Detection

Detected via FTC inquiry letter. We had no internal monitoring for potential compliance violations.

**Improvements:** Established quarterly compliance audits and added automated checks for age-related data collection.

## Prevention

- Mandatory legal review for all new products (POL-001: Tethics Framework)
- Age verification required for any product collecting personal information
- Compliance training for all engineers during onboarding
- Regular third-party compliance audits
