## Agent skills

### Issue tracker

Issues and PRDs are tracked in GitHub Issues for `timjonaswechler/star_sim`. See `docs/agents/issue-tracker.md`.

### Triage labels

The repository uses the five default canonical triage labels. See `docs/agents/triage-labels.md`.

### Domain docs

The repository uses a single-context domain documentation layout. See `docs/agents/domain.md`.


# Naming Rules
If any kind of symbol (function, module, struct, enum, etc.), folder or file have the same prefix like `some_functionA`, `some_functionB`, `some_functionC`, etc., they should be grouped together into their one  hierarchy higher representation. For example, `some_functionA`, `some_functionB`, `some_functionC` should be grouped together into a module named `some` and their functions can live named `functionA` and `functionB` and `functionC`.

## Commits

- Use Conventional Commits: `<type>[optional scope][!]: <description>`.
- Use `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`, `style`, or `revert`.
- Keep commits atomic, independently buildable and testable; separate unrelated changes.
- Write a lowercase description without a trailing period; keep the header within 100 characters.
- Mark breaking changes with `!` and explain them in a `BREAKING CHANGE:` footer.
- Reference an issue only when the commit implements, fixes, or directly follows up on it.
- Put issue references in the commit footer, for example `Refs #31`.
- Use `Fixes #42` or `Closes #42` only when merging the commit should close that open issue.
- A genuine follow-up may reference a closed issue with `Refs #31`.
- Omit the issue reference when no issue directly applies.
- Only create commits when explicitly requested.
