# Create Decision Record

Capture a business decision using the SPADE framework.

## When to Use

- Business strategy decisions
- Resource allocation choices
- Process changes
- Vendor/tool selections
- Policy decisions

## Workflow

1. **Check for conflicts**: Search existing records for related decisions
   ```bash
   dg search "<topic>"
   ```

2. **Create the record**:
   ```bash
   dg new decision "<title>"
   ```

3. **Edit the created file** to fill in:
   - **Setting**: Context and stakes
   - **People**: SPADE roles (Responsible, Approver, Consulted, Informed)
   - **Alternatives**: At least 2-3 options with pros/cons
   - **Decision**: Which option and why
   - **Consequences**: Positive, negative, follow-ups

4. **Link to related records**:
   ```bash
   dg link DEC-XXX depends_on STR-YYY
   dg link DEC-XXX implements OPP-ZZZ
   ```

5. **Update status** when approved:
   ```bash
   dg status DEC-XXX accepted
   ```

## Example

User: "We decided to use PostgreSQL instead of MongoDB for the new service"

→ Create: `dg new decision "Use PostgreSQL for user service"`
→ Link: `dg link DEC-XXX implements ADR-YYY` (if there's a related ADR)
→ Status: `dg status DEC-XXX accepted`
