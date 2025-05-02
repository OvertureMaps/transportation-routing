# Commit Message Guidelines

This document outlines the proper format for commit messages in our repository.

## Format

Commit messages should follow this format:

```
Short summary (50 chars or less)

More detailed explanatory text, if necessary. Wrap it to about 72
characters. The blank line separating the summary from the body is
critical.

- Bullet points are okay
- Use a hyphen or asterisk followed by a space

If applicable, reference issues and pull requests:
Fixes #123
Related to #456
```

## Guidelines

1. **Start with a concise summary line**:
   - 50 characters or less
   - Begin with a capital letter
   - Do not end with a period
   - Use imperative mood ("Add feature" not "Added feature")

2. **Follow with a detailed description** (if needed):
   - Separate from summary by a blank line
   - Explain what and why, not how
   - Wrap text at ~72 characters
   - Use bullet points for multiple items

3. **Reference issues and PRs** at the end if applicable

## Using a Template File

For complex commit messages, use a template file:

1. Create a file with your commit message:
   ```bash
   $ vim commit-message.txt
   ```

2. Write your message following the format above

3. Commit using the file:
   ```bash
   $ git commit -F commit-message.txt
   ```

This approach avoids issues with escaped newlines and ensures proper formatting.

## Examples

### Good:

```
Add support for Overture Graph Tiles

This commit implements the initial structure for Overture Graph Tiles:

- Creates basic tile format definition
- Adds serialization/deserialization functions
- Implements unit tests for tile operations

The implementation follows the design outlined in the RFC and
provides the foundation for the transcoder development.

Fixes #42
```

### Bad:

```
fixed stuff
```

```
Added new feature that implements the tile format for the Overture Graph Tiles project which will allow us to convert Overture data to a format that can be used by routing engines and this is a really long commit message that doesn't wrap properly.
```
