# Go: Complete TDD Cycle for Next Test

Execute the complete TDD cycle (Red → Green → Refactor) for the next unmarked test in plan.md, following Kent Beck's TDD principles.

## Instructions

When you receive `/go`, you should:

1. **Find the next unmarked test** in plan.md
2. **RED Phase**: Write a failing test
3. **GREEN Phase**: Implement minimum code to make test pass
4. **REFACTOR Phase**: Improve code structure if needed
5. **Mark test as complete** in plan.md

## Complete TDD Cycle

```
1. Read plan.md and find next unmarked test ([ ])
   ↓
2. RED: Write the simplest failing test
   - Use meaningful test name
   - Make test failure clear and informative
   ↓
3. GREEN: Implement minimum code to pass
   - Write just enough code - no more
   - Use simplest solution that works
   ↓
4. Run all tests - confirm they pass
   ↓
5. REFACTOR: Improve structure if needed
   - Only refactor when tests pass
   - Make one refactoring at a time
   - Run tests after each refactoring
   ↓
6. Mark test as complete ([x]) in plan.md
   ↓
7. Update Current Status in plan.md
```

## TDD Principles (from CURSOR.md)

- Always write **one test at a time**
- Always make it **run** (pass)
- Always **improve structure** before moving on
- Always run **all tests** each time (except long-running tests)
- Write the **simplest failing test** first
- Implement the **minimum code** needed to make tests pass
- Refactor only **after tests are passing**

## Test Selection

From plan.md, select the next test that:
- Is marked as `[ ]` (unmarked)
- Is the simplest, smallest test case
- Focuses on one behavior at a time
- Builds incrementally on previous tests

## Implementation Guidelines

### RED Phase
- Write a test that defines a small increment of functionality
- Use meaningful test names that describe behavior
- Make test failures clear and informative
- Test should fail for the right reason

### GREEN Phase
- Write the simplest code that makes the test pass
- It's okay to hardcode values or use naive implementations
- Don't worry about code quality yet - that comes in Refactor phase
- Focus on making the current test pass, nothing more

### REFACTOR Phase
- Only refactor when tests are passing
- Make structural changes without changing behavior
- Eliminate duplication ruthlessly
- Express intent clearly through naming and structure
- Make one refactoring change at a time
- Run tests after each refactoring step

## Tidy First Approach

When both structural and behavioral changes are needed:
1. Make **structural changes first** (Tidy First)
2. Validate structural changes do not alter behavior
3. Then make **behavioral changes**
4. Never mix structural and behavioral changes

## After Completion

- Mark the test as `[x]` in plan.md
- Update "Current Status" section with completed test
- Indicate next test to work on
- All tests should be passing
- Code should be clean and well-structured

## Remember

- Follow the TDD cycle precisely: Red → Green → Refactor
- Always prioritize clean, well-tested code over quick implementation
- Write one test at a time, make it run, then improve structure
- Always run all tests (except long-running tests) each time
