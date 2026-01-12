# Complete TDD Cycle

Follow the complete **TDD cycle** (Red → Green → Refactor) for implementing new functionality.

## Step-by-Step Process

### 1. RED: Write a Failing Test
- Write the simplest failing test for a small part of the feature
- Use meaningful test names that describe behavior
- Make test failures clear and informative
- Run the test to confirm it fails

### 2. GREEN: Make the Test Pass
- Implement the bare minimum code to make the test pass
- Use the simplest solution that could possibly work
- Don't worry about code quality yet
- Run tests to confirm they pass (Green)

### 3. REFACTOR: Improve the Code
- Make structural changes if needed (Tidy First)
- Run tests after each change to ensure nothing broke
- Eliminate duplication
- Improve clarity and expressiveness
- Keep methods small and focused

## Full Workflow

```
1. Write simple failing test (RED)
   ↓
2. Implement minimum code (GREEN)
   ↓
3. Run all tests - do they pass?
   ↓ Yes
4. Any structural improvements needed? (Tidy First)
   ↓ Yes → Make structural changes → Run tests → Commit structural changes
   ↓ No
5. Any more functionality needed?
   ↓ Yes → Go to step 1
   ↓ No
6. Commit behavioral changes
```

## Key Principles

- **One test at a time**: Always write one test, make it run, then improve structure
- **Run all tests** (except long-running tests) each time
- **Separate structural from behavioral changes**
- **Small, frequent commits** rather than large, infrequent ones
- **Always prioritize clean, well-tested code** over quick implementation

