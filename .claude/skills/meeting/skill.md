# Create Meeting Notes

Capture meeting discussions, decisions, and action items.

## When to Use

- Architecture reviews
- Sprint planning
- Decision-making meetings
- Brainstorming sessions
- Post-mortems
- Any meeting worth documenting

## Workflow

1. **Create the record**:
   ```bash
   dg new meeting "<title>"
   ```

2. **Fill in during/after meeting**:
   - **Attendees**: Who was there
   - **Agenda**: What was planned
   - **Discussion**: Key points per topic
   - **Decisions Made**: What was decided (with owners)
   - **Action Items**: Tasks with owners and due dates
   - **Next Steps**: Follow-up meetings, etc.

3. **Link to related records**:
   ```bash
   dg link MTG-XXX enables DEC-YYY  # If a decision was made
   dg link MTG-XXX relates_to ADR-ZZZ
   ```

4. **Create follow-up records** for decisions made:
   ```bash
   dg new decision "Decision from meeting"
   dg link DEC-XXX depends_on MTG-YYY
   ```

## Example

User: "We had an architecture review meeting today"

→ Create: `dg new meeting "Architecture review 2024-01-15"`
→ Document attendees, discussion, decisions
→ For major decisions: create separate DEC or ADR records
→ Link: `dg link MTG-XXX enables ADR-YYY`
