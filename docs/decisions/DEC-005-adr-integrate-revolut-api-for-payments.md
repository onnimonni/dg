---
type: decision
id: DEC-005
title: "ADR: Integrate Revolut API for Payments"
status: proposed
created: 2026-01-27
updated: 2026-01-27
authors: [engineering]
tags: [adr, api, architecture, integration, payments, revolut]
links:
  implements: [DEC-004]
---

# ADR: Integrate Revolut API for Payments

This is an Architecture Decision Record for the technical implementation of Revolut integration.

## Context

Following DEC-004 (Switch to Revolut for SEK Banking), we need to integrate Revolut's API into our payment system for automated reconciliation of Swedish payments.

## Decision Drivers

- Automated payment reconciliation required
- Must handle SEK and EUR currencies
- Need real-time webhook notifications
- Must maintain audit trail for compliance

## Considered Options

### Option A: Direct Revolut Business API
Use Revolut's official Business API with OAuth2 authentication.

**Pros:**
- Official supported integration
- Real-time webhooks
- Full feature access
- Good documentation

**Cons:**
- Revolut-specific implementation
- Need to handle OAuth token refresh

### Option B: Open Banking (PSD2)
Use standard Open Banking APIs to access Revolut.

**Pros:**
- Bank-agnostic implementation
- Could switch providers easily

**Cons:**
- Limited feature set
- More complex authentication
- No webhooks

### Option C: Manual Export/Import
Download CSV from Revolut, import into our system.

**Pros:**
- No development needed
- Works immediately

**Cons:**
- Manual process
- Delayed reconciliation
- Error prone

## Decision

Chosen: **Option A: Direct Revolut Business API**

We will use Revolut's Business API with:
- OAuth2 for authentication
- Webhooks for real-time transaction notifications
- Batch API calls for daily reconciliation backup

## Technical Details

### Authentication Flow
```
1. Initial OAuth2 authorization (manual, one-time)
2. Store refresh token securely (encrypted at rest)
3. Auto-refresh access token before expiry
```

### Webhook Events
Subscribe to:
- `TransactionCreated`
- `TransactionStateChanged`

### Fallback
Daily batch job at 02:00 UTC fetches all transactions as backup.

## Consequences

### Positive
- Real-time payment visibility
- Automated reconciliation
- Reduced manual work

### Negative
- Vendor lock-in to Revolut API
- Need to monitor API deprecations
- OAuth token management complexity

### Follow-up Actions
- [ ] Request Revolut API sandbox access
- [ ] Implement OAuth2 flow
- [ ] Set up webhook endpoint
- [ ] Build reconciliation service
- [ ] Create monitoring dashboards
