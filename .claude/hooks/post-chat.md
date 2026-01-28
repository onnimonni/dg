# Decision Graph Post-Chat Hook

After completing a conversation, consider whether any decisions, strategies, or important information should be captured in the Decision Graph.

## Triggers for Capture

### Decisions
- User made or confirmed a business decision
- Chose between alternatives
- Approved/rejected a proposal
- Changed direction from previous approach

### Strategies
- Discussed company direction
- Planned market entry, expansion, or pivot
- Set goals or objectives
- Defined tenets or principles

### Policies
- Established rules or guidelines
- Discussed compliance requirements
- Set operational boundaries
- Created standards

### Client Information
- Discussed specific client needs
- Documented customizations
- Recorded client requirements or constraints
- Noted client history or issues

### Opportunities
- Identified market gap
- Discovered customer need
- Found potential for improvement
- Validated or invalidated hypothesis

### Processes
- Defined how something should be done
- Established workflow
- Created procedures
- Set responsibilities

## Actions

1. **Check for existing records**: `dg search "relevant terms"`
2. **Create new record if needed**: `dg new TYPE "title"`
3. **Link to related records**: `dg link FROM relates_to TO`
4. **Update status if needed**: `dg status ID accepted`

## What NOT to Capture

- Temporary discussions without conclusion
- Personal preferences without business impact
- Technical implementation details (use ADRs for those)
- Confidential information that shouldn't be in version control
