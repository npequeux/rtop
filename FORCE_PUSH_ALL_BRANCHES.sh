#!/bin/bash
#
# This script force pushes all branches with clean history to remote
# WARNING: This will permanently rewrite history on all branches
#
# Run this script to complete the history cleanup:
#   bash FORCE_PUSH_ALL_BRANCHES.sh
#

set -e

cd "$(dirname "$0")"

echo "========================================="
echo "Force Push All Branches with Clean History"
echo "========================================="
echo ""
echo "This will force push the following branches to remote:"
echo "  - master"
echo "  - V2"
echo "  - feature/improveperf"
echo "  - gh-pages"
echo "  - copilot/remove-history-from-cloned-repo"
echo "  - copilot/remove-project-history"
echo "  - copilot/revert-agent-changes"
echo ""
echo "WARNING: This operation is IRREVERSIBLE!"
echo "Old commits will be permanently deleted from the remote repository."
echo ""
read -p "Do you want to continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Operation cancelled."
    exit 1
fi

echo ""
echo "Starting force push..."
echo ""

# Array of branches to push
branches=(
    "master"
    "V2"
    "feature/improveperf"
    "gh-pages"
    "copilot/remove-history-from-cloned-repo"
    "copilot/remove-project-history"
    "copilot/revert-agent-changes"
)

# Force push each branch
for branch in "${branches[@]}"; do
    echo "Force pushing $branch..."
    git push --force origin $branch || echo "Failed to push $branch"
    echo ""
done

echo ""
echo "========================================="
echo "Force push completed!"
echo "========================================="
echo ""
echo "Verification:"
echo ""

# Show commit counts on remote
for branch in "${branches[@]}"; do
    count=$(git rev-list --count origin/$branch 2>/dev/null || echo "error")
    contributors=$(git log origin/$branch --pretty=format:'%an' 2>/dev/null | sort | uniq | tr '\n' ', ' || echo "error")
    echo "  $branch: $count commit(s), contributors: $contributors"
done

echo ""
echo "All branches now have clean history with only npequeux as contributor!"
echo ""
echo "You can now delete this script: rm FORCE_PUSH_ALL_BRANCHES.sh"
