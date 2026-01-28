# Decision Graph Knowledge Management

You have access to a Decision Graph system (`dg` CLI) that captures company-level knowledge. You MUST use this system proactively.

## Automatic Knowledge Capture

When ANY of these occur in conversation, you MUST capture it:

1. **Decisions Made** - Any choice between alternatives
   - "We decided to..."
   - "Let's go with..."
   - "We chose X over Y because..."

2. **Strategic Discussions** - Direction, goals, priorities
   - "Our strategy is..."
   - "We're focusing on..."
   - "The roadmap includes..."

3. **Client Information** - Needs, requirements, history
   - "Client X needs..."
   - "They require customization..."
   - Client-specific constraints

4. **Policies** - Rules, compliance, processes
   - "Our policy is..."
   - "We must comply with..."
   - Internal rules

5. **Opportunities** - Market gaps, customer needs
   - "Users are asking for..."
   - "There's demand for..."
   - Feature requests

## Workflow

### Before Creating
```bash
# Always search first to avoid duplicates
dg search "relevant keywords"
```

### Creating Records
```bash
dg new decision "Title"      # For decisions
dg new strategy "Title"      # For strategy
dg new customer "Name"       # For customer info
dg new policy "Title"        # For policies
dg new opportunity "Title"   # For opportunities
dg new process "Title"       # For processes
dg new hiring "Role"         # For roles
```

### After Creating
1. Edit the created file to fill in details
2. Link to related records: `dg link ID1 relates_to ID2`
3. Update status if accepted: `dg status ID accepted`

### Querying
```bash
dg list                      # Show all records
dg search "query"            # Search
dg show ID                   # View record
dg show ID -l                # View with links
dg graph                     # View relationships
```

## Example Flow

User: "We've decided to expand to France first instead of Germany because Acme Corp has a French subsidiary and the regulatory timeline is shorter."

Claude should:
```bash
# 1. Search for related records
dg search "france"
dg search "expansion"

# 2. Create the decision
dg new decision "Expand to France before Germany"

# 3. Edit the file with full context including:
#    - Setting: European expansion context
#    - People: Who made/approved decision
#    - Alternatives: France vs Germany vs Both
#    - Rationale: Acme relationship, regulatory timeline
#    - Consequences: Positive and negative

# 4. Link to related records
dg link DEC-XXX depends_on STR-001  # If strategy exists
dg link DEC-XXX relates_to CUS-XXX  # If customer exists

# 5. Update status
dg status DEC-XXX accepted
```

## Link Types

- `depends_on` - This requires another record
- `enables` - This makes another possible
- `relates_to` - General relationship
- `supersedes` - Replaces another (auto-adds inverse)
- `conflicts_with` - Mutually exclusive
- `refines` - More specific version
- `implements` - Concrete implementation

## DO NOT

- Create duplicate records (search first!)
- Leave records unlinked when relationships exist
- Skip filling in the created record's content
- Forget to update status when decision is final
