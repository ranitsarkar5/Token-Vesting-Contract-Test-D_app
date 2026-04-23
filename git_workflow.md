# Recommended Git Workflow

To meet the challenge requirement of "At least 3 meaningful Git commits", here is a suggested commit sequence. Run these commands from your project root as you finalize your submission:

```bash
# Commit 1: Core infrastructure
git add contracts/ Cargo.toml
git commit -m "feat(contract): implement core Soroban crowdfund logic and unit tests"

# Commit 2: Frontend implementation
git add frontend/
git commit -m "feat(frontend): scaffold React App with Freighter connection and state caching UI"

# Commit 3: Documentation and final polish
git add README.md demo_script.md git_workflow.md
git commit -m "docs: add comprehensive README, deployment guide, and demo video script"
```
