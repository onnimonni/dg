# Get Decision Graph Context

Search the decision graph for related records before making changes.

## When to Use

- Before implementing a feature
- When discussing architecture
- When a topic seems familiar
- When you're unsure if something was decided
- When the user mentions a customer, decision, or process

## Workflow

1. **Search for related records**:
   ```bash
   dg search "<topic>"
   ```

2. **Filter by type if needed**:
   ```bash
   dg list --type adr
   dg list --type decision
   dg list --type customer
   ```

3. **View specific records**:
   ```bash
   dg show DEC-001
   dg show ADR-015
   ```

4. **See the relationship graph**:
   ```bash
   dg graph DEC-001  # Show records connected to DEC-001
   ```

5. **Check for conflicts**:
   - If you find related records, READ them
   - If they conflict with current discussion, ASK the user
   - If they support current work, REFERENCE them

## When to Ask for Clarification

Ask the user when:
- A new decision conflicts with an existing one
- A customer request contradicts existing architecture
- A proposed change would supersede existing decisions
- You're unsure if something was already decided

Example: "I found ADR-015 which says we should use PostgreSQL, but you're asking about MongoDB. Should we update the ADR or is this a different use case?"

## Example

User: "Let's add caching to the API"

→ Search: `dg search "caching"` or `dg search "redis"` or `dg search "performance"`
→ If found: "I found ADR-003 which discusses our caching strategy. Let me check if this aligns..."
→ If not found: Proceed, but consider creating an ADR for the new caching decision
