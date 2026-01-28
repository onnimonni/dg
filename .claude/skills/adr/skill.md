# Create Architecture Decision Record

Capture a technical/architecture decision.

## When to Use

- Technology stack choices
- API design decisions
- Database schema decisions
- Infrastructure choices
- Security architecture
- Performance trade-offs

## Workflow

1. **Check for conflicts**: Search existing ADRs
   ```bash
   dg search "<topic>" --type adr
   ```

2. **Create the record**:
   ```bash
   dg new adr "<title>"
   ```

3. **Edit the created file** to fill in:
   - **Context**: What problem are we solving?
   - **Decision**: What did we decide?
   - **Consequences**: Positive, negative, neutral
   - **Alternatives Considered**: What else did we evaluate?

4. **Link to related records**:
   ```bash
   dg link ADR-XXX implements DEC-YYY
   dg link ADR-XXX depends_on ADR-ZZZ
   dg link ADR-XXX relates_to CUS-WWW
   ```

5. **Update status**:
   ```bash
   dg status ADR-XXX accepted
   ```

## Example

User: "Let's use Redis for caching instead of Memcached"

→ Create: `dg new adr "Use Redis for application caching"`
→ Fill in context (performance needs), alternatives (Memcached, no cache)
→ Link: `dg link ADR-XXX implements DEC-YYY` (if driven by a business decision)
