# Tidy First: Behavioral Changes

You are making **BEHAVIORAL CHANGES** following Kent Beck's Tidy First approach.

## Instructions

1. **Ensure structural changes are complete** and committed separately
2. **Follow the TDD cycle** (Red → Green → Refactor) for behavioral changes
3. **Behavioral changes include**:
   - Adding new functionality
   - Modifying existing behavior
   - Fixing bugs
   - Changing how the code works (not just how it looks)
4. **Write a failing test first** (Red phase)
5. **Implement minimum code** to pass the test (Green phase)
6. **Refactor if needed** after tests pass

## Critical Rules

- **NEVER mix structural and behavioral changes** in the same commit
- **Always follow TDD cycle** for behavioral changes
- **Make behavioral changes only after** structural changes are done
- Commit behavioral changes separately from structural ones

## Commit Discipline

Only commit when:
- ALL tests are passing
- ALL compiler/linter warnings have been resolved
- The change represents a single logical unit of work
- Commit message clearly states this is a behavioral change

## What to do next

Continue with TDD cycle or move to refactoring phase as needed.

