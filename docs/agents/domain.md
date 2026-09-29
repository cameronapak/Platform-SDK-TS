# Domain docs

How engineering skills should consume this repository's domain documentation.

## Before exploring

Read these sources when they exist:

- `CONTEXT.md` at the repository root.
- Relevant architecture decision records under `docs/adr/`.

If these files do not exist, proceed silently. Do not treat their absence as an error or create them preemptively. Domain-modeling skills create them when vocabulary or decisions are resolved.

## Layout

This is a single-context repository:

```text
/
├── CONTEXT.md
├── docs/adr/
└── src/
```

## Use the glossary's vocabulary

When an issue title, proposal, hypothesis, or test names a domain concept, use the term defined in `CONTEXT.md`. Do not drift to a synonym that the glossary explicitly avoids.

If a needed concept is absent, reconsider whether the term belongs to the domain. If it does, note the gap for domain modeling.

## Flag ADR conflicts

If proposed work contradicts an existing architecture decision record, surface the conflict explicitly rather than silently overriding the decision.
