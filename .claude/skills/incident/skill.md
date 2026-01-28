# Create Incident Report

Capture a post-mortem / incident report using blameless format.

## When to Use

- Production outages
- Security incidents
- Data loss events
- Service degradation
- Near-misses worth documenting

## Workflow

1. **Create the record immediately**:
   ```bash
   dg new incident "<title>"
   ```

2. **Fill in the timeline** as events unfold:
   - When did it start?
   - When was it detected?
   - When was it mitigated?
   - When was it resolved?

3. **After resolution**, complete:
   - **Impact**: Users affected, duration, revenue impact
   - **Root Cause**: Use 5 Whys if helpful
   - **What went well/poorly**
   - **Action items** with owners and due dates

4. **Link to related records**:
   ```bash
   dg link INC-XXX relates_to ADR-YYY
   dg link INC-XXX enables RUN-ZZZ  # If a runbook was created
   ```

5. **Update status**:
   ```bash
   dg status INC-XXX resolved
   ```

## Example

User: "The database went down for 2 hours yesterday"

→ Create: `dg new incident "Database outage 2024-01-15"`
→ Fill timeline, root cause, action items
→ Link to any ADRs that need updating
→ Create runbooks for prevention: `dg new runbook "Database failover procedure"`
