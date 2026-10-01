## Agent skills

### Issue tracker

Issues and specs live in this repository's GitHub Issues. See `docs/agents/issue-tracker.md`.

### Domain docs

This repository uses a single-context domain-doc layout. See `docs/agents/domain.md`.

## SDK generation

Prefer Cloudflare Forge and Fern's pinned generators, supported configuration, and native runtime patterns. Reusing their defaults keeps handwritten code small and lets generation own the API surface. Keep language SDKs idiomatic; do not force identical APIs or retry timing across languages.

The YouVersion Platform API contract takes precedence over generator defaults. Verify behavior through installed-consumer tests. When a default fails that contract, prefer supported configuration, then the smallest assertion-backed adaptation. Keep the authoritative OpenAPI unchanged for generator convenience, and make regeneration fail on unexpected output drift. Do not replace a generated runtime or upgrade shared pins without explaining the demonstrated need and affected targets.
