# Build ADR kind of tool which would work for company level decision making

1. Explore for a good format which would help store central company level information in basic text files
2. Build cli with go or rust which would be able to create and alter these docs and build interconnectivity between them like ADRs do.
3. Build Claude skill and subagent and good EXAMPLE-CLAUDE.md which will guide how to use this.

Intention is to add claude hook which would ask claude to both save the decisions and be able to search for decisions related to company strategy and clients.

Ideally it would be able to document questions like:
* Why did we expand to France instead of Germany?
* Key client needs customization X into our software
* Regulatory needs force us to keep PII out of certain decision making
* Workforce laws require us to do X so we have policy Y
* Customer study noticed that people are lacking featury Z

This way claude could grow and maintain pool of relevant information in structured way inside a monorepo which would be able to define the direction for a whole company.

## More on structured documents
I’m familiar with ADRs.

Are there other well structured standards to define companys opportunities, problems, recruiting process, clients or something else than just the architecture?
Yes. Just as ADRs (Architecture Decision Records) prevent "architectural amnesia" by documenting the context and why behind code changes, other domains have developed similar immutable records to stabilize chaos in hiring, strategy, and product development.

Here are the most well-structured standards that function as the "ADR equivalent" for other areas of business.

1. For Company Strategy & Problems: The "Six-Pager"

Origin: Amazon The Concept: PowerPoint decks are banned in favor of narrative memos. Just like an ADR forces you to write down the "Consequences," a Six-Pager forces you to write down the complete narrative before a meeting starts.

Structure:

Context/Introduction: What is the current state?

Goals: What specifically are we trying to achieve?

Tenets: What principles guide this decision? (Similar to "drivers" in ADRs).

State of the Business: Data-backed view of reality.

Strategic Priorities: The actual plan.

FAQ: Anticipated questions and tough answers.

Why it works: It forces "high-resolution" thinking. You cannot hide weak logic behind bullet points, much like you can't hide bad code design in a detailed ADR.

2. For Complex Decisions: SPADE

Origin: Square (Gokul Rajaram) The Concept: A framework for making difficult decisions (hiring a V-level exec, entering a new market) where consensus is impossible. It creates a "Decision Record" for the business.

Structure:

S - Setting: The "Context" of an ADR. What, when, and why?

P - People: Who is the Responsible one (makes the call), who Approves (veto power only), and who is Consulted.

A - Alternatives: Three viable options (not just one real option and two straw men).

D - Decide: The decision is made and recorded.

E - Explain: The communication plan to the company.

Why it works: It clarifies who owns the decision, preventing the "meeting after the meeting."

3. For Product & Opportunities: Opportunity Solution Trees (OST)

Origin: Teresa Torres The Concept: Instead of a roadmap (which is just a list of features), OSTs are a visual record of the path from a clear business problem to a specific solution.

Structure:

Outcome: The metric you want to move (e.g., "Increase retention by 10%").

Opportunity: The customer pain point or need (e.g., "I can't find my old reports").

Solution: The feature idea (e.g., "Add a search bar").

Experiment: How you tested it.

Why it works: It prevents "feature factories." If a feature (Solution) doesn't link back to a validated Problem (Opportunity), you don't build it.

4. For Recruiting: The "Scorecard" (Structured Hiring)

Origin: Who (Geoff Smart & Randy Street) / Google The Concept: Most hiring is based on "vibes." Structured hiring creates a "Hiring Decision Record" based on data.

Structure:

Mission: A short summary of the role's purpose.

Outcomes: 3–8 specific, measurable goals the person must achieve by year 1 (e.g., "Close $1M in sales").

Competencies: The specific behaviors required to achieve those outcomes.

Ratings: Every interviewer rates the candidate on specific competencies, not general "likability."

Why it works: It decouples the definition of the role from the assessment of the candidate, reducing bias.

5. For Project Management: DACI

Origin: Atlassian / Intuit The Concept: A lightweight matrix to define roles for a project, ensuring everyone knows their "lane."

Structure:

D = Driver: The person herding the cats (Project Manager).

A = Approver: The one person who makes the final call.

C = Contributors: People doing the work.

I = Informed: People you tell after the decision is made.

Comparison Table