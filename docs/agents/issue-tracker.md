# Issue tracker: GitHub

Issues and specs for this repository live as GitHub issues. Use the `gh` CLI for all operations.

## Conventions

- **Create an issue**: `gh issue create --title "..." --body "..."`. Use a heredoc for multi-line bodies.
- **Read an issue**: `gh issue view <number> --comments`, including its labels.
- **List issues**: `gh issue list --state open --json number,title,body,labels,comments` with appropriate label and state filters.
- **Comment on an issue**: `gh issue comment <number> --body "..."`.
- **Apply or remove labels**: `gh issue edit <number> --add-label "..."` or `--remove-label "..."`.
- **Close an issue**: `gh issue close <number> --comment "..."`.

Infer the repository from `git remote -v`; `gh` does this automatically when run inside the clone.

## Pull requests as a triage surface

**PRs as a request surface: no.** Set this to `yes` if this repository later treats external pull requests as feature requests.

GitHub shares one number space across issues and pull requests. Resolve an ambiguous `#42` with `gh pr view 42`, then fall back to `gh issue view 42`.

## Skill instructions

- When a skill says "publish to the issue tracker," create a GitHub issue.
- When a skill says "fetch the relevant ticket," run `gh issue view <number> --comments`.

## Wayfinding operations

When a skill organizes work as a map and child tickets:

- Use one issue labelled `wayfinder:map` as the map.
- Link tickets as GitHub sub-issues. If sub-issues are unavailable, use a task list in the map and add `Part of #<map>` to each child.
- Represent blocking relationships with GitHub's native issue dependencies. If dependencies are unavailable, add `Blocked by: #<number>` to the child.
- A ticket is ready when it is open, unassigned, and has no open blocker.
- Claim work by assigning the ticket to the current user.
- Resolve work by commenting with the outcome and closing the ticket.
