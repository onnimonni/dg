# Create Runbook

Capture step-by-step operational instructions.

## When to Use

- Deployment procedures
- Maintenance tasks
- Incident response steps
- Credential rotation
- Database migrations
- Any repeatable operational task

## Workflow

1. **Create the record**:
   ```bash
   dg new runbook "<title>"
   ```

2. **Fill in the template**:
   - **Purpose**: When and why to use this
   - **Prerequisites**: What's needed before starting
   - **Steps**: Numbered, with exact commands
   - **Verification**: How to confirm success
   - **Rollback**: How to undo if needed
   - **Troubleshooting**: Common problems and solutions

3. **Link to related records**:
   ```bash
   dg link RUN-XXX implements PRC-YYY  # If part of a process
   dg link RUN-XXX relates_to INC-ZZZ  # If created after incident
   ```

## Best Practices

- Include exact commands (copy-paste ready)
- Add expected output after each step
- Include timing expectations
- Note when manual verification is needed
- Keep steps atomic and reversible

## Example

User: "How do we rotate the API keys?"

→ Create: `dg new runbook "Rotate API keys"`
→ Fill in step-by-step commands
→ Add verification: "curl the health endpoint"
→ Add rollback: "Restore previous keys from vault"
