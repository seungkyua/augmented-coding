# Fix a Defect Using TDD

When fixing a defect, follow Kent Beck's TDD approach with a two-step test strategy.

## Instructions

### Step 1: Write an API-Level Failing Test
1. **Write a failing test at the API/interface level**
   - This test should demonstrate the desired behavior from the user's perspective
   - Use the public interface/API of your code
   - Make it clear what the expected behavior should be

### Step 2: Write the Smallest Failing Test
2. **Write the smallest possible test that replicates the problem**
   - This test should isolate the specific defect
   - It should fail in the same way the defect manifests
   - Focus on the exact condition that causes the issue

### Step 3: Make Both Tests Pass
3. **Implement the fix to make both tests pass**
   - Fix the defect with minimum necessary changes
   - Ensure both the API-level test and the specific test pass
   - Don't add unnecessary functionality

### Step 4: Verify and Refactor
4. **Run all tests to ensure nothing broke**
5. **Refactor if needed** (only after all tests pass)

## Process Flow

```
1. Write API-level failing test (defines desired behavior)
   ↓
2. Write smallest failing test (replicates the defect)
   ↓
3. Implement fix to make both tests pass
   ↓
4. Run all tests - confirm everything passes
   ↓
5. Refactor if needed (Tidy First)
   ↓
6. Run tests again
   ↓
7. Commit the fix
```

## Key Principles

- **Test from outside in**: API-level test first, then specific test
- **Fix the defect, nothing more**
- **Ensure all tests pass** before refactoring
- **Commit fix separately** from any structural changes

## Example

If you have a bug where "user login fails with invalid password":

1. API test: `test_user_cannot_login_with_invalid_password()` - demonstrates desired behavior
2. Specific test: `test_password_validation_rejects_empty_string()` - isolates the exact issue
3. Fix the password validation
4. Both tests should pass

