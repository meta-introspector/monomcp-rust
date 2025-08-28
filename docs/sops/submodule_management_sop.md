# Standard Operating Procedure: Submodule Management (Build, Test, Push)

## 1. Purpose
This SOP outlines the process for managing Git submodules within the `monomcp` project, specifically focusing on building, testing, and pushing changes to their respective remote repositories, and subsequently updating the superproject. This procedure ensures consistency, proper versioning, and successful integration of submodule changes.

## 2. Scope
This SOP applies to all Git submodules within the `monomcp` repository that require independent building, testing, and pushing of changes.

## 3. Prerequisites
*   A local clone of the `monomcp` repository, including initialized and updated submodules.
*   Appropriate build tools installed for each submodule's language/framework (e.g., Rust toolchain for Rust projects).
*   Network connectivity to access remote Git repositories.
*   Proper authentication/permissions to push to submodule remote repositories.

## 4. Procedure

### 4.1. Identify Modified Submodules
Before starting, identify which submodules have local changes or new commits that need to be pushed.

```bash
git status
```
Look for lines indicating `modified: <submodule_path> (new commits)` or `modified: <submodule_path> (modified content)`.

### 4.2. Process Each Modified Submodule

For each submodule identified in Step 4.1:

#### 4.2.1. Navigate to Submodule Directory
```bash
cd monomcp/<submodule_name>
```

#### 4.2.2. Check Submodule Status
Verify the exact state of the submodule.
```bash
git status
```
*   If "Your branch is ahead of 'origin/main' by X commits": There are local commits to push.
*   If "Changes not staged for commit" or "Untracked files": There are uncommitted changes. These must be committed within the submodule before pushing.

#### 4.2.3. Commit Local Changes (if any)
If `git status` shows uncommitted changes, stage and commit them within the submodule. Provide a clear and concise commit message.

```bash
git add .
git commit -m "Descriptive commit message for submodule changes"
```

#### 4.2.4. Build the Submodule
Execute the appropriate build command for the submodule. For Rust projects, this is typically:

```bash
cargo build --release
```
Address any build errors before proceeding.

#### 4.2.5. Test the Submodule
Execute the appropriate test command for the submodule. For Rust projects, this is typically:

```bash
cargo test
```
Ensure all tests pass. Address any test failures before proceeding.

#### 4.2.6. Push Submodule Changes
Push the submodule's committed changes to its remote repository.

```bash
git push origin <branch_name> # e.g., git push origin main
```
Replace `<branch_name>` with the actual branch name of the submodule (usually `main` or `master`).

#### 4.2.7. Return to Superproject Root
```bash
cd ../.. # Adjust based on submodule nesting level
```

### 4.3. Update Superproject
After successfully building, testing, and pushing changes for all relevant submodules, the superproject needs to record the new commit hashes of these submodules.

#### 4.3.1. Stage Submodule Changes in Superproject
```bash
git add monomcp/<submodule_name> # Repeat for each updated submodule
```

#### 4.3.2. Commit Superproject Changes
Commit the updated submodule references in the superproject.

```bash
git commit -m "Update submodules: <submodule1>, <submodule2>, ..."
```

#### 4.3.3. Push Superproject Changes
Finally, push the superproject's changes to its remote.

```bash
git push origin main # Or your main branch name
```

## 5. Troubleshooting
*   **Build/Test Failures:** Review error messages, check submodule-specific documentation, and ensure all dependencies are met.
*   **Push Failures:** Verify network connectivity, remote repository URL, and authentication credentials.
*   **Submodule "dirty" status persists:** Ensure all changes within the submodule were committed and pushed, and that the superproject was updated.

## 6. Automation (Future Work)
This manual process will serve as the basis for future automation efforts, potentially using a Rust-based MCP Git plugin to streamline submodule management.
