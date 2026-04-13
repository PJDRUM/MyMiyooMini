# AGENTS.md

## Project goal
Build a minimalist Miyoo Mini launcher.

Treat this as a launcher/front-end project only.
Do not treat it as a firmware, kernel, driver, or low-level systems project.

## Product priorities
1. Fast boot to game selection
2. Minimal UI
3. Stable game launching
4. Minimal in-game exit flow

## Explicit non-goals
- Do not add features unless they directly support the priorities above.
- Do not add dependencies unless absolutely required.
- Do not change unrelated files.
- Do not spend time on UI polish unless explicitly requested.

## Working rules
When making changes:
1. Inspect the existing architecture first.
2. Propose the smallest viable change.
3. Test with available simulator, build, or validation commands when present.
4. Report the exact files changed.

## Scope guardrails
- Optimize for speed of reaching the game list, not menus or settings depth.
- Prefer fewer screens, fewer prompts, and fewer moving parts.
- Preserve reliable launch behavior over convenience features.
- Keep the in-game exit flow minimal and predictable.
- If a request starts drifting into firmware or platform internals, stop and redirect back to launcher/front-end behavior.

## Change style
- Favor small, localized edits.
- Reuse existing patterns before introducing new structure.
- Avoid speculative abstractions.
- Keep implementation and UI behavior simple by default.
