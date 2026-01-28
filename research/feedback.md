# Feedback: Microsoft Decision Graph & DG Tool

## 1. Feedback on Microsoft Documents

The decision graph successfully captures the high-level strategic pivots of Microsoft, but it could be richer in several areas to fully leverage the "Decision Graph" concept.

### Strengths
- **Narrative Arc**: The progression from Founding -> DOS -> Windows -> Internet is clear.
- **Why, not just What**: The "Alternatives" sections effectively capture *why* decisions were made (e.g., the QDOS buy vs. build tradeoff).
- **Linking**: The dependency chain (founding enables DOS, DOS enables Windows, etc.) is logically sound.

### Areas for Improvement
- **Granularity**: The current graph is "Strategic only." It misses the tactical decisions that make the story interesting.
    - *Example*: The "Look and Feel" lawsuit from Apple is missing.
    - *Example*: The decision to license Windows 95 to any OEM (commoditizing hardware) vs. keeping it closed.
    - *Example*: The "Halloween Memo" regarding Linux and open source.
- **Relationships**: We mostly used `depends_on`. We should use:
    - `conflicts_with`: To show internal tensions (e.g., Windows vs. OS/2 development teams).
    - `supersedes`: To show how new strategies replaced old ones (e.g., "Internet Tidal Wave" superseding "MSN Proprietary Network").
- **Tags**: The tags are generic (`strategy`, `product`). Better tags would be `antitrust`, `partnership`, `licensing` to allow filtering by business leveraged.
- **Failures**: The graph is a "history of winners." It lacks the failures like "Microsoft Bob," "Zune," or the "Kin" phone, which are instructive decisions that had creating/learning value.

## 2. Feedback on the `dg` Tool

Based on the usage experience, here are suggestions to improve the tool's usability and developer experience.

### CLI & Configuration
- **Confusing Argument Parsing**:
    - *Issue*: `cargo run --bin dg -- -c ./samples/microsoft/dg.toml` failed with `unexpected argument '-c'`.
    - *Suggestion*: Standardize on `--config` or `-c` for pointing to the configuration file, or auto-detect `dg.toml` in parent directories.
    - *Current Behavior*: It seems to prioritize `-D` (docs dir) but the relationship between the config file location and the docs directory is rigid (expects them to be adjacent or standard).
- **Run Directory Sensitivity**:
    - *Issue*: Running `dg build` from the project root using `-D` required correct relative paths that felt brittle. Running from the directory itself was smoother.
    - *Suggestion*: The tool should be location-agnostic. If I point to a directory, it should treat that as the "project root" for the build.

### Content / Schema
- **Validation**:
    - *Observation*: It wasn't immediately clear if `authors` in markdown frontmatter *must* match keys in `dg.toml`.
    - *Suggestion*: `dg validate` command should warn if a decision references an author id that isn't defined in the config.

### Features
- **Visual Graph**:
    - *Observation*: The generated graph is likely a static view.
    - *Suggestion*: For complex histories like Microsoft, a "Timeline View" (Gantt style) would be as valuable as a dependency graph.
- **Context Awareness**:
    - *Idea*: Use the "Setting" section of previous decisions to auto-fill the context for future decisions (LLM helper feature).

## 3. Next Steps for Microsoft Graph
To make this a "Showcase" example:
1.  **Add the "Browser Wars" Node**: Explicitly model the decision to bundle IE freely, which `conflicts_with` standard antitrust compliance.
2.  **Add "Steve Ballmer's Sales Force"**: The decision to build a massive enterprise sales team (distinct from Bill's product focus).
3.  **Add "Xbox"**: The decision to enter the living room (hardware), which `conflicts_with` the pure software business model.
