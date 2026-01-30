# Cognitive Architectures for Decision Support Systems: A Comprehensive Taxonomy of Mental Models

## Executive Summary

The modern human condition is defined by an unprecedented abundance of choice and a scarcity of cognitive bandwidth. This paradox creates a specific market need for decision support tools—systems that do not merely provide answers, but provide the *structures* through which answers can be derived. The user's intent to build a brainstorming tool rooted in established frameworks addresses a critical gap in the current landscape of productivity software: the need for context-aware metacognition. While raw information is ubiquitous, the "mindware" required to process it is often lacking.

This report serves as a foundational architectural document for such a tool. It moves beyond the typical listicle format of self-help advice to provide a rigorous, academic, and actionable analysis of cognitive frameworks. We have categorized the human experience into five primary domains of inquiry—**Ideation & Identity**, **Valuation & Acquisition**, **Restoration & Physiology**, **Connection & Conflict**, and **Strategy & Growth**.

For each domain, we analyze specific mental models (e.g., The Odyssey Plan, The Diderot Effect, Spoon Theory, The Sound Relationship House) derived from behavioral economics, cognitive psychology, and systems theory. We dissect their mechanisms, their theoretical origins, and, crucially, their algorithmic translation for digital tool design. This report provides the necessary intellectual capital to construct a sophisticated decision-making engine capable of guiding users from confusion to clarity.

------

## Part I: The Theoretical Basis of Cognitive Frameworks

### 1.1 The Necessity of Externalized Cognition

The human brain functions primarily as a pattern-matching engine, evolved for survival in a linear, local environment. It is ill-equipped for the non-linear, global, and abstract complexities of modern life. When a user asks, "What hobby should I start?" or "Why am I tired?", they are not merely asking for data; they are reporting a failure of their internal processing models to handle the variables at play.

Mental models serve as "cognitive scaffolding." Just as a physical scaffold allows a worker to reach heights otherwise inaccessible, a mental model allows the thinker to reach conclusions that raw intuition (System 1 thinking) cannot support. For a digital tool, the objective is to externalize this scaffolding. The tool must act as a surrogate prefrontal cortex, imposing structure on the chaotic inputs of the user's life.

### 1.2 The Taxonomy of User States

To design an effective brainstorming tool, one must first classify the "state" of the user. Our research indicates that queries generally fall into specific psychological clusters, each requiring a distinct class of framework:

1. **The Generative State:** Characterized by a need for expansion and possibility (e.g., "What hobby?", "What career?").
2. **The Evaluative State:** Characterized by a need for convergence and selection (e.g., "Should I buy this?", "Make or buy?").
3. **The Depleted State:** Characterized by a resource deficit (e.g., "I am tired", "I am burned out").
4. **The Relational State:** Characterized by interpersonal complexity (e.g., "I am lonely", "We are fighting").
5. **The Strategic State:** Characterized by a desire for optimization (e.g., "How do I learn?", "How do I grow?").

The following sections detail the specific frameworks applicable to each state, providing the logic required to program the tool's decision trees.

------

## Part II: The Generative Engine – Frameworks for Ideation and Identity

When a user approaches a tool with the question, "What hobby should I choose?" or "What should I do with my life?", they are typically suffering from the Paradox of Choice. The objective of the Generative Engine is not to provide an infinite list of options, but to filter the universe of possibilities through the constraints of the user's values and reality.

### 2.1 Ikigai: The Intersection of Purpose and Utility

The concept of *Ikigai* is perhaps the most universally recognized framework for life design, yet it is frequently misunderstood in Western contexts. For the purposes of a decision tool, understanding this duality—between the Westernized "Venn Diagram" model and the authentic Japanese philosophy—is critical for tailored advice.

#### 2.1.1 The Westernized Framework (The Venn Diagram)

The popular Western interpretation of Ikigai acts as a strategic planning tool for career and major life pivots. It visualizes the intersection of four distinct circles:

| **Component**                | **Definition**                                     | **Strategic Question for the User**              |
| ---------------------------- | -------------------------------------------------- | ------------------------------------------------ |
| **What You Love**            | Activities that induce 'flow' and intrinsic joy.   | "What activities make you lose track of time?"   |
| **What You Are Good At**     | Skills, talents, and cultivated expertise.         | "What do people ask you for help with?"          |
| **What The World Needs**     | Societal gaps, community needs, or market demands. | "What problems in the world upset you the most?" |
| **What You Can Be Paid For** | Market viability and monetization potential.       | "What skills of yours have a market rate?"       |

**Algorithmic Translation:**

The tool should guide the user to populate these four lists. The intersections create specific "zones" of experience:

- **Passion:** Intersection of *Love* and *Good At*.
- **Profession:** Intersection of *Good At* and *Paid*.
- **Vocation:** Intersection of *Paid* and *World Needs*.
- **Mission:** Intersection of *World Needs* and *Love*.

True *Ikigai* in this model is the center point where all four overlap. However, the tool must recognize edge cases. If a user has Passion and Mission but no payment, they have "Delight and fullness, but no wealth." If they have Profession and Vocation but no love, they have "Comfortable, but a feeling of emptiness".

#### 2.1.2 The Authentic Japanese Nuance (The Non-Monetary Framework)

Research indicates that the authentic Japanese concept of Ikigai does not necessarily involve economic gain or "saving the world". It is often found in small, daily rituals—the "joy of little things." It is less about a grand career purpose and more about *connectability* and *presence*.

**Tool Design Insight:** When a user asks about "hobbies" rather than "careers," the Western model's "What You Can Be Paid For" circle becomes a liability. It introduces "hustle culture" into leisure. A sophisticated tool should detect the context. If the user query is "hobby," the tool should switch to a modified **3-Circle Ikigai** (Love, Skill, World/Community) or a **Resilience-Based Ikigai** that focuses on "What supports your emotional resilience?". The tool should explicitly warn users against the pressure to monetize every interest.

### 2.2 Odyssey Planning: Prototyping Future Lives

Developed by Bill Burnett and Dave Evans at Stanford’s d.school, Odyssey Planning applies the principles of Design Thinking to life choices. It is a powerful antidote to "stuckness" because it rejects the premise that there is only one "correct" life path.

#### 2.2.1 The Three-Timeline Logic

The framework requires the generation of three distinct 5-year timelines. This is not a prediction exercise, but an *ideation* exercise.

1. **Plan A: The Expected Path.** This is the linear extension of the user's current life. If they are an accountant, this plan involves becoming a senior accountant.
2. **Plan B: The Alternative Path.** This forces a constraint: "What if Plan A vanished?" If the accounting industry were automated by AI tomorrow, what would the user do? This forces the brain to access dormant skills and interests.
3. **Plan C: The Wild Card Path.** This removes constraints of money and status. "If you knew you could not fail and money was no object, what would you do?" This reveals deep, often suppressed desires.

#### 2.2.2 The Dashboard Metrics

For a digital tool, simply listing these plans is insufficient. The tool must ask the user to rate each plan on four gauges :

- **Resources:** (0-100%) Do you have the time, money, and network?
- **Likability:** (0-100%) How much does the thought of this excite you?
- **Confidence:** (0-100%) How sure are you that you can achieve this?
- **Coherence:** (0-100%) Does this fit with your broader worldview and values?

**Algorithmic Translation:** The tool should present these gauges as sliders. A high "Likability" but low "Resources" score on Plan C suggests the user needs a "bridge" hobby or a long-term saving strategy, rather than an immediate career jump. This framework is particularly useful for users who feel trapped, as it mathematically demonstrates the existence of options.

### 2.3 The "Next Next Job" Framework

While Odyssey Planning is broad, the "Next Next Job" framework (popularized by Andrew Chen) is a tactical tool for career sequencing. It addresses the common user dilemma of comparing disparate job offers.

**The Mechanism:**

Instead of evaluating a potential opportunity based on its immediate benefits (salary, title), the user evaluates it based on its *positioning power* for the role *after* that.

1. **Define the Terminal Goal:** (e.g., "I want to be a CMO").
2. **Identify Gaps:** "What does a CMO have that I lack?" (e.g., P&L responsibility, team management).
3. **Evaluate Current Options:** "Which of these available jobs solves for the 'P&L' gap?"

**Tool Design Insight:** This framework introduces the concept of *backward chaining*  to the tool. When a user asks "Which job should I take?", the tool should immediately query: "What is the job you want 5-10 years from now?" If the user cannot answer, the tool routes them to **Odyssey Planning** first. If they can, it applies the **Next Next Job** filter.

### 2.4 The Anti-Bucket List and Inversion

Brainstorming is often inhibited by the pressure to find "good" ideas. The **Inversion Principle**, a favorite of stoics and mathematicians like Jacobi ("Invert, always invert"), suggests that avoiding stupidity is easier than seeking brilliance.

**The Anti-Bucket List:** Instead of listing "Things I want to do," the user lists "Things I refuse to do" or "Things I have no interest in," regardless of their social popularity.

- *Examples:* "I do not want to commute more than 30 minutes," "I do not want to manage a large team," "I do not care about visiting Disney World."

**Algorithmic Translation:** This acts as a negative filter. Before generating hobby or career ideas, the tool asks the user to define the "Anti-Criteria." This significantly reduces the search space and prevents the generation of "false positives"—ideas that sound good on paper but violate the user's intrinsic boundaries. For example, if a user lists "No cold weather" on their Anti-List, suggestions like "Skiing" or "Ice Sculpting" are immediately pruned, increasing the trust in the tool's recommendations.

------

## Part III: The Evaluative Engine – Frameworks for Purchasing and Selection

Once the user has options, the cognitive load shifts from *divergent thinking* (creation) to *convergent thinking* (selection). Queries in this domain often take the form: "Should I buy this?", "Which of these three items is best?", or "Is this worth the money?"

### 3.1 The Weighted Decision Matrix (Pugh Matrix)

For complex decisions with multiple variables (e.g., choosing a hobby, buying a house, selecting a vendor), the human brain struggles to weigh factors simultaneously. The Decision Matrix externalizes this processing.

**The Architecture:**

1. **Rows (Options):** The items being compared (e.g., Painting vs. Coding vs. Hiking).
2. **Columns (Criteria):** The factors that matter (e.g., Cost, Social Connection, Skill Growth, Fun).
3. **Weights (Importance):** A multiplier (1-5) assigned to each criterion based on user values.

**Calculation Logic:**

$$\text{Total Score} = \sum (\text{Option Rating} \times \text{Criteria Weight})$$

**Critique & Nuance:** A naive implementation of a Decision Matrix can lead to "false precision," where the user manipulates the weights to get the answer they subconsciously want.

- **Tool Feature:** To counter this, the tool should include a "Sensitivity Analysis" mode. It asks: "If 'Cost' was half as important, would the winner change?" This reveals the robustness of the decision. Additionally, the tool should include a "Gut Check" phase—if the matrix picks 'Hiking' and the user feels disappointed, the tool should flag this discrepancy as a data point in itself.

### 3.2 Cost-Benefit Analysis (CBA) and The ROI of Intangibles

CBA is the standard framework for binary "Yes/No" decisions. However, in a personal context, its power lies in quantifying the *intangible*.

**The Four Quadrants:**

1. **Tangible Costs:** Money, physical space.
2. **Intangible Costs:** Learning curve, maintenance effort, mental load, social friction.
3. **Tangible Benefits:** Time saved, money generated.
4. **Intangible Benefits:** Joy, status, peace of mind, health.

**Algorithmic Translation:**

The tool must force the user to assign a dollar value (or an "Energy Unit" value) to intangibles.

- *Example:* A user considers a robot vacuum ($500).
  - Tangible Cost: $500.
  - Tangible Benefit: 1 hour saved per week.
  - *Tool Query:* "How much is one hour of your weekend worth?" If the user says $50, the ROI break-even is 10 weeks. This converts abstract "worth" into concrete "time".

### 3.3 Cost Per Use (CPU)

This framework is essential for combating the cognitive bias of "sticker shock" versus "value." It is particularly useful for clothing, gadgets, and hobby equipment.

**Formula:**

$$\text{CPU} = \frac{\text{Purchase Price} + \text{Maintenance Cost}}{\text{Estimated Lifetime Uses}}$$

**Algorithmic Translation:**

The tool should present a "True Cost Calculator."

- *Scenario:* A user debates between a $50 pair of boots (lasting 1 year, 50 wears) and $300 boots (lasting 10 years, 500 wears).
  - Option A: $1.00 per wear.
  - Option B: $0.60 per wear. The tool visualizes that the "expensive" option is actually the "cheaper" option over time. This reframes the decision from "Spending" to "Amortizing."

### 3.4 The Diderot Effect and System Coherence

The Diderot Effect describes a consumption spiral where purchasing a new, high-quality item makes one's existing possessions seem inadequate, leading to a chain reaction of upgrades. It is named after the philosopher Denis Diderot, who was gifted a beautiful scarlet robe, which then made his desk look cheap, so he replaced the desk, then the rug, then the chair, until he was "master of the robe, but slave to the furniture."

**Tool Design Insight:**

When a user wants to start a new hobby or make a "departure purchase" (something unlike their current belongings), the tool must trigger a **Diderot Warning**.

- *Prompt:* "You are buying a high-end DSLR camera. List the accessories you will *feel* compelled to buy to justify this purchase (lenses, tripod, bag, editing software)."
- *Goal:* To calculate the "Ecosystem Cost," not just the "Item Cost." This helps the user decide if they are ready to upgrade their entire "system" or if they should buy a product that fits their current reality (e.g., a point-and-shoot).

### 3.5 The Hedonic Treadmill and Affective Forecasting

The Hedonic Treadmill is the observed tendency of humans to return to a relatively stable level of happiness despite major positive or negative life changes. This implies that the "joy" of a purchase is transient.

**Algorithmic Translation:**

To counter the bias of "focusing illusion" (overestimating how much a purchase will change one's life), the tool should employ a **Time-Travel Prompt**.

- *Query:* "Imagine it is 6 months from now. The item is scratched and the novelty has worn off. Do you still use it daily? Does it still bring you joy?"
- *Application:* This forces the user to evaluate the *utility* of the item rather than the *dopamine hit* of the acquisition.

### 3.6 The 30-Day Rule (Impulse Control)

This is a temporal friction framework. The rule states: "If you want a non-essential item, wait 30 days. If you still want it, buy it."

**Tool Feature:** A "Wishlist Locker." The user inputs an item they want to buy. The tool "locks" this idea for 30 days and sends a notification when the timer expires. This utilizes the cooling-off period to let the emotional brain (amygdala) subside and the rational brain (prefrontal cortex) take over. For smaller items, a 24-hour or 72-hour rule may be applied.

------

## Part IV: The Restorative Engine – Frameworks for Energy and Physiology

"I am tired" is one of the most common, yet least understood, user queries. Fatigue is a multi-dimensional phenomenon, yet most people treat it with a uni-dimensional solution: sleep. The Restorative Engine is designed to diagnose the *type* of fatigue and prescribe the *correct* remedy.

### 4.1 The 7 Types of Rest Framework

Dr. Saundra Dalton-Smith’s research revolutionizes the understanding of fatigue by categorizing it into seven distinct deficits. A user suffering from "Creative Deficit" will not be cured by "Physical Rest" (sleeping); in fact, sleeping might make them feel more lethargic.

**Table: The 7 Types of Rest Diagnostic**

| **Rest Type** | **Symptoms of Deficit**                                  | **Restorative Framework/Activity**                           |
| ------------- | -------------------------------------------------------- | ------------------------------------------------------------ |
| **Physical**  | Muscle aches, heavy limbs, sheer exhaustion.             | **Passive:** Sleep, napping. **Active:** Yoga, stretching, massage. |
| **Mental**    | Brain fog, irritability, inability to focus.             | Short breaks, "worry dumping" (journaling), meditation.      |
| **Sensory**   | Overwhelmed by lights/noise, "Zoom fatigue."             | Silence, darkness, digital detox, closing eyes for 2 mins.   |
| **Creative**  | Lack of ideas, feeling "stuck," problem-solving fatigue. | Nature walks, art appreciation, play, allowing wonder.       |
| **Emotional** | People-pleasing, hiding true feelings.                   | Vulnerability, venting to a safe friend, therapy.            |
| **Social**    | Feeling drained by interactions (even Zoom).             | Solitude (for introverts) or spending time with "life-giving" people. |
| **Spiritual** | Feeling unanchored, cynical, lack of purpose.            | Prayer, meditation, volunteering, connecting to the "bigger picture." |

**Algorithmic Translation:** The tool cannot simply say "Go to sleep." It must run a diagnostic quiz: "Do you feel muscle fatigue or brain fog?" "Are you irritable or uninspired?" Based on the answers, it maps the user to the specific Rest Type. For a "Sensory" deficit, the recommendation might be "Turn off your phone for 1 hour," whereas for "Creative" deficit, it might be "Go for a walk in a park without headphones".

### 4.2 The Personal Energy Audit (Loehr & Schwartz)

Derived from *The Power of Full Engagement*, this framework shifts the focus from "Time Management" to "Energy Management". It posits that energy, not time, is the fundamental currency of high performance.

**The Four Dimensions:**

1. **Body (Physical):** Quantity of energy (Glucose, sleep, fitness).
2. **Emotions (Quality):** Quality of energy (Negative vs. Positive).
3. **Mind (Focus):** Focus of energy (Time management, attention).
4. **Spirit (Force):** Force of energy (Meaning, purpose).

**The Audit Mechanism:** The tool should guide the user through a "Gain/Drain" analysis.

- *Step 1:* List activities from the last week.
- *Step 2:* Label each as an "Energy Gain" (recharge) or "Energy Drain" (deplete).
- *Step 3:* Analyze the ratio.
- *Insight:* High performance is not linear; it is oscillatory. Humans are designed to pulse between expenditure and recovery. The audit reveals if the user is living linearly (constant drain).

### 4.3 Spoon Theory (Capacity Management)

Originally a metaphor for chronic illness (Lupus), Spoon Theory has become a vital mental model for burnout and capacity management.

- **The Concept:** A person starts the day with a finite number of "spoons" (units of energy). Every task costs a spoon: getting out of bed (1 spoon), showering (1 spoon), a stressful meeting (3 spoons).
- **The Crisis:** When spoons are gone, they are gone. Pushing past zero borrows from tomorrow's spoons at a high interest rate.

**Tool Design Insight:**

For users identifying as "burned out" or "chronically ill," the tool should switch from "Productivity Mode" (optimizing for output) to "Spoon Mode" (optimizing for conservation).

- *Feature:* A "Daily Spoon Budget." The user assigns spoon costs to their to-do list. If the total exceeds their daily limit, the tool ruthlessly prioritizes, forcing the user to defer tasks. This validates the user's limited capacity rather than shaming them for it.

### 4.4 HALT (Impulse & Regulation Diagnostic)

Used widely in addiction recovery, HALT is a rapid diagnostic for emotional volatility.

- **H**ungry
- **A**ngry
- **L**onely
- **T**ired

**Application:**

When a user wants to make a rash decision (quit a job, send an angry email, buy an expensive item), the tool should trigger a **HALT Check**.

- *Prompt:* "Before you proceed, let's check your baselines. When was the last time you ate? How did you sleep?"
- *Logic:* Often, what feels like an existential crisis is merely a physiological deficit. The tool acts as a biological interlock.

### 4.5 The Explore/Exploit Tradeoff

This framework, rooted in computer science and evolutionary biology, helps decide between trying new things and sticking with known winners.

- **Explore:** Gathering information (trying a new hobby, reading a new book). Costs energy; high variance.
- **Exploit:** Using known information (rewatching a favorite movie, eating at a favorite restaurant). Saves energy; guaranteed reward.

**Connection to Tiredness:**

If a user is tired (low energy), the algorithm should suggest **Exploitation** strategies (comfort, familiarity). If the user is bored (stagnant energy), it should suggest **Exploration**.

- *Tool Logic:* "You reported feeling drained. This is not the time to start learning the violin (Explore). This is the time to re-read your favorite novel (Exploit)".

------

## Part V: The Strategic Engine – Frameworks for Career, Habits, and Growth

### 5.1 Ramit Sethi’s Money Dials

This framework challenges the traditional "latte factor" frugality. It posits that budgeting is not about restriction, but about allocation based on values.

- **The Philosophy:** "Spend extravagantly on the things you love, and cut costs mercilessly on the things you don't."

**The 10 Money Dials:**

1. Convenience
2. Travel
3. Health/Fitness
4. Experiences
5. Freedom
6. Relationships
7. Generosity
8. Luxury
9. Social Status
10. Self-Improvement

**Algorithmic Translation:**

The tool asks the user to identify their primary Money Dial.

- *Scenario:* User selects "Convenience."
- *Recommendation:* The tool advises *spending* money on a cleaner, grocery delivery, or a direct flight, while simultaneously advising *cutting* spending on "Luxury" (cars, watches) or "Travel" if those are low priorities. This creates a "Guilt-Free Spending" roadmap.

### 5.2 The Dreyfus Model of Skill Acquisition

When a user asks "How do I learn X?", the advice must be tailored to their proficiency level. The Dreyfus Model explains that learners have different needs at different stages.

| **Stage**                | **Characteristic**                        | **Learning Need**                                  | **Tool Recommendation**                           |
| ------------------------ | ----------------------------------------- | -------------------------------------------------- | ------------------------------------------------- |
| **1. Novice**            | Adheres strictly to rules; no context.    | "Recipes," strict algorithms, step-by-step guides. | "Follow this tutorial exactly. Do not deviate."   |
| **2. Advanced Beginner** | Starts to see situational elements.       | Guidelines, maxims, simple troubleshooting.        | "Here are the common mistakes to avoid."          |
| **3. Competent**         | Conscious planning; emotional investment. | Planning frameworks, decision trees.               | "Create a project plan using these parameters."   |
| **4. Proficient**        | Intuitive understanding; holistic view.   | Case studies, complex scenarios.                   | "Analyze this case study and propose a solution." |
| **5. Expert**            | Unconscious competence.                   | Freedom, experimentation.                          | "Experiment with breaking the rules."             |

**Tool Design Insight:** The tool must assess user level. If a Novice asks for "Coding advice," providing abstract principles ("Write clean code") is useless. They need strict syntax rules. If an Expert asks, strict rules are insulting. The tool must dynamically adjust the *granularity* of its advice.

### 5.3 Atomic Habits (The 4 Laws)

James Clear’s framework is the operational system for implementing any of the ideas generated in the other sections.

**The 4 Laws of Behavior Change:**

1. **Cue:** Make it Obvious.
2. **Craving:** Make it Attractive.
3. **Response:** Make it Easy.
4. **Reward:** Make it Satisfying.

**Algorithmic Translation (The Habit Debugger):**

When a user says, "I can't stick to my new hobby," the tool runs a diagnostic based on the *Inversion* of these laws.

- *Diagnostic:* "Is the cue invisible?" (Did you hide your guitar in the closet?) -> *Fix:* Put it in the living room.
- *Diagnostic:* "Is the response difficult?" (is the gym 20 mins away?) -> *Fix:* Go to a closer gym or workout at home. This moves habit formation from "willpower" to "environmental design".

------

## Part VI: The Relational Engine – Frameworks for Connection and Conflict

Social queries ("I'm lonely", "My partner is annoying me") are high-stakes. The tool must use frameworks that de-escalate emotion and structure communication.

### 6.1 The Sound Relationship House (Gottman)

Dr. John Gottman’s research provides a structural blueprint for healthy relationships.

**Key Components for Tool Logic:**

- **Build Love Maps:** Does the user know their partner's current stressors?
- **Turn Towards:** Is the user responding to "bids" for connection? (e.g., Partner says "Look at that bird," User ignores. This is a failed bid).
- **Manage Conflict:** Is the user engaging in the "Four Horsemen" (Criticism, Contempt, Defensiveness, Stonewalling)?

**Tool Feature:** A "Conflict Audit." The user inputs a recent argument. The tool scans for "You statements" (Criticism) and suggests rephrasing. It checks if the "Magic Ratio" (5 positive interactions for every 1 negative) is being violated.

### 6.2 The Friendship Formula

Developed by Jack Schafer (ex-FBI), this formula demystifies social connection.

$$\text{Friendship} = \text{Proximity} + \text{Frequency} + \text{Duration} + \text{Intensity}$$

**Insight for the "Lonely" User:**

Users often obsess over "Intensity" (being funny/smart) but neglect the variables of Proximity and Frequency.

- *Tool Advice:* "Don't try to be interesting. Just show up to the same coffee shop (Proximity) at the same time (Frequency) for a month." This leverages the *Mere Exposure Effect* to build trust before intensity is required.

### 6.3 Non-Violent Communication (NVC)

Marshall Rosenberg’s NVC is a protocol for verbalizing needs without triggering defensiveness.

**The 4-Step Script:**

1. **Observation:** Pure fact (video camera recording). "You arrived at 6:15." (Not "You were late").
2. **Feeling:** Emotion. "I felt anxious." (Not "I felt ignored"—ignored is a judgment of the other's action).
3. **Need:** Universal value. "I need reassurance/safety."
4. **Request:** Concrete action. "Please text me if you are running 15 minutes behind."

**Tool Feature:** The "NVC Translator." The user types: "My boss is a jerk who never listens." The tool translates: "Observation: During the meeting, I spoke and was interrupted. Feeling: Frustrated. Need: To be heard/Respect. Request: Can we agree to let each person finish speaking?".

### 6.4 Deciding to End a Relationship

When the user asks "Should I break up?", the tool must navigate carefully.

- **The Two-Chair Technique:** Visualize sitting in one chair (Stay) and then the other (Leave). Which feels authentic?.
- **Triandis Model:** Analyze the decision based on Habits, Intentions, and Social Norms.
- **Values Check:** Are the deal-breakers (kids, religion, location) structural? If so, no amount of NVC will fix it.

------

## Part VII: The Learning Engine – Frameworks for Understanding

### 7.1 The Feynman Technique

For users asking "How do I understand this complex topic?", Richard Feynman’s algorithm is the gold standard.

**The 4 Steps:**

1. **Choose Concept:** Write it down.
2. **Teach it to a Child:** Explain it in simple language. No jargon allowed.
3. **Identify Gaps:** When you stumble or use jargon, you have identified a gap in your own knowledge. Go back to the source material.
4. **Simplify:** Create an analogy.

**Tool Design:** A "Feynman Editor." The user types an explanation. The tool highlights complex words and jargon, flagging them as "Knowledge Gaps" and prompting the user to define them simply. If they can't, the tool directs them to study that specific sub-topic.

------

## Part VIII: Tool Implementation & Architecture

To satisfy the user's request to "create a tool," we must synthesize these frameworks into a cohesive system architecture. The tool is effectively a **State-Machine for Metacognition**.

### 8.1 The Routing Logic

The tool initiates with a "Triage" phase to determine the user's domain.

| **User Input / Signal**       | **Detected Domain** | **Primary Framework**                              | **Secondary Framework**       |
| ----------------------------- | ------------------- | -------------------------------------------------- | ----------------------------- |
| "I'm bored", "Need ideas"     | **Generative**      | Anti-Bucket List (Filter) -> Odyssey Plan (Ideate) | Ikigai (Refine)               |
| "Which one?", "Should I?"     | **Evaluative**      | Decision Matrix (Complex) / CBA (Binary)           | 30-Day Rule (Impulse)         |
| "I'm tired", "Stressed"       | **Restorative**     | HALT (Triage) -> 7 Types of Rest (Diagnosis)       | Spoon Theory (Budget)         |
| "Fight with spouse", "Lonely" | **Relational**      | NVC (Scripting)                                    | Friendship Formula (Strategy) |
| "Start business", "Learn X"   | **Strategic**       | 7 Powers (Strategy) / Dreyfus (Learning)           | Atomic Habits (Execution)     |

### 8.2 The "Energy Gating" Mechanism

A critical missing feature in most tools is context-awareness regarding user energy.

- *Logic:* If a user is in a "Restorative" state (HALT-Tired), the tool *must not* offer high-cognitive-load frameworks like the Decision Matrix or Odyssey Plan.
- *Action:* It should block those features and route the user to "Passive Rest" or "Exploitation" strategies (e.g., "Watch a movie"). Only once the energy state is resolved should the tool unlock the "Generative" or "Strategic" modules. This prevents the tool from exacerbating decision fatigue.

### 8.3 The "Anti-Framework" Warning System

The tool must prevent "Category Errors"—using the wrong framework for the domain.

- *Error:* Using **Cost-Benefit Analysis** for a **Relationship** decision.
- *Warning:* "You are attempting to optimize a relationship using transactional logic. This often leads to resentment. Would you like to try the 'Sound Relationship House' framework instead?"
- *Error:* Using **Generative Brainstorming** when **Tired**.
- *Warning:* "Your energy audit suggests depletion. Brainstorming now will likely produce low-quality ideas. We recommend a 'Creative Rest' break first."

## Conclusion

The architecture of choice is not about finding the perfect answer; it is about finding the perfect *question*. The frameworks detailed in this report—from Ikigai to NVC, from Spoon Theory to the Feynman Technique—are essentially sophisticated question-generating engines.

By building a tool that embeds these models, we do not just automate decision-making; we upgrade the user's operating system. We move them from reactive, instinctual choices (System 1) to proactive, structured, and strategic life design (System 2). In a world of infinite noise, the ultimate luxury is a clear mental model. This report provides the blueprints for that luxury.