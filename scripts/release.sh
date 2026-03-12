#!/usr/bin/env bash
set -euo pipefail

# ── Colors & Symbols ──────────────────────────────────────────────
BOLD='\033[1m'
DIM='\033[2m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
RESET='\033[0m'

ARROW="${CYAN}▸${RESET}"
CHECK="${GREEN}✔${RESET}"
CROSS="${RED}✖${RESET}"

STEP_NUM=0

step() {
  STEP_NUM=$((STEP_NUM + 1))
  echo ""
  echo -e "${BOLD}${BLUE}[${STEP_NUM}/7]${RESET} ${BOLD}$1${RESET}"
}

info() {
  echo -e "  ${ARROW} $1"
}

success() {
  echo -e "  ${CHECK} $1"
}

error() {
  echo -e "  ${CROSS} ${RED}$1${RESET}" >&2
}

# Spinner for long-running commands
spin() {
  local pid=$1
  local label=$2
  local frames=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
  local i=0

  while kill -0 "$pid" 2>/dev/null; do
    printf "\r  ${CYAN}${frames[$i]}${RESET} ${DIM}%s${RESET}" "$label"
    i=$(( (i + 1) % ${#frames[@]} ))
    sleep 0.1
  done
  wait "$pid"
  local exit_code=$?
  printf "\r\033[K"
  return $exit_code
}

# Run a command with spinner
run_with_spinner() {
  local label=$1
  shift
  "$@" &
  local pid=$!
  if spin "$pid" "$label"; then
    success "$label"
  else
    error "$label — failed"
    return 1
  fi
}

# ── Args ──────────────────────────────────────────────────────────
LEVEL="${1:?Usage: release.sh <patch|minor|major> [--dry-run]}"
DRY_RUN="${2:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [[ "$LEVEL" != "patch" && "$LEVEL" != "minor" && "$LEVEL" != "major" ]]; then
  error "level must be patch, minor, or major"
  exit 1
fi

# ── Preflight ─────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}${CYAN}unifast release${RESET} ${DIM}(${LEVEL})${RESET}"
echo -e "${DIM}─────────────────────────────────────${RESET}"

if [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
  error "Working directory is not clean. Commit or stash changes first."
  exit 1
fi

BRANCH="$(git -C "$ROOT" branch --show-current)"
if [[ "$BRANCH" != "main" ]]; then
  error "Releases must be made from main branch (current: ${BRANCH})"
  exit 1
fi

CURRENT_VERSION="$(sed -n '/\[workspace\.package\]/,/\[/{s/^version = "\(.*\)"/\1/p}' "$ROOT/Cargo.toml")"
if [[ -z "$CURRENT_VERSION" ]]; then
  error "Could not read version from Cargo.toml"
  exit 1
fi

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
case "$LEVEL" in
  patch) PATCH=$((PATCH + 1)) ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
esac
NEXT_VERSION="${MAJOR}.${MINOR}.${PATCH}"

echo ""
echo -e "  ${YELLOW}${CURRENT_VERSION}${RESET} → ${GREEN}${BOLD}${NEXT_VERSION}${RESET}"

# ── Dry Run ───────────────────────────────────────────────────────
if [[ "$DRY_RUN" == "--dry-run" ]]; then
  echo ""
  echo -e "${BOLD}${YELLOW}  DRY RUN — no changes will be made${RESET}"
  echo ""
  echo -e "  ${BOLD}Files to update:${RESET}"
  echo -e "    ${ARROW} Cargo.toml ${DIM}(workspace version)${RESET}"
  for pkg in "$ROOT"/packages/*/package.json; do
    if echo "$pkg" | grep -q "benchmark"; then continue; fi
    local_pkg="${pkg#"$ROOT"/}"
    echo -e "    ${ARROW} ${local_pkg}"
  done
  echo ""
  echo -e "  ${BOLD}CHANGELOG preview:${RESET}"
  git-cliff --unreleased --tag "v${NEXT_VERSION}" 2>/dev/null || echo -e "    ${DIM}(no conventional commits found)${RESET}"
  exit 0
fi

# ── Step 1: Bump versions ────────────────────────────────────────
step "Bump versions"

sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEXT_VERSION\"/" "$ROOT/Cargo.toml"
sed -i "s/\(unifast-core = {.*version = \"\)$CURRENT_VERSION\"/\1$NEXT_VERSION\"/" "$ROOT/Cargo.toml"
success "Cargo.toml"

for pkg in "$ROOT"/packages/*/package.json; do
  if echo "$pkg" | grep -q "benchmark"; then continue; fi
  sed -i "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$NEXT_VERSION\"/" "$pkg"
  local_pkg="${pkg#"$ROOT"/}"
  success "${local_pkg}"
done

# ── Step 2: Update Cargo.lock ─────────────────────────────────────
step "Update Cargo.lock"

cargo check --manifest-path "$ROOT/Cargo.toml" 2>&1 \
  | grep -E '^(Compiling|Checking|Updating|Downloading|Locking)' \
  | while IFS= read -r line; do
      info "${DIM}${line}${RESET}"
    done || true

success "Cargo.lock synced"

# ── Step 3: Generate CHANGELOG ────────────────────────────────────
step "Generate CHANGELOG"

if command -v git-cliff &>/dev/null; then
  run_with_spinner "git-cliff" git-cliff --tag "v${NEXT_VERSION}" -o "$ROOT/CHANGELOG.md"
else
  info "${YELLOW}git-cliff not found — skipping CHANGELOG generation${RESET}"
  info "Install: ${DIM}cargo install git-cliff${RESET}"
fi

# ── Step 4: Stage changes ────────────────────────────────────────
step "Stage changes"

git -C "$ROOT" add -A
CHANGED=$(git -C "$ROOT" diff --cached --stat | tail -1)
success "${CHANGED}"

# ── Step 5: Commit & tag ─────────────────────────────────────────
step "Commit & tag"

git -C "$ROOT" commit -m "release: v${NEXT_VERSION}"
success "Committed ${GREEN}release: v${NEXT_VERSION}${RESET}"

git -C "$ROOT" tag -a "v${NEXT_VERSION}" -m "v${NEXT_VERSION}"
success "Tagged ${GREEN}v${NEXT_VERSION}${RESET}"

# ── Step 6: Push ──────────────────────────────────────────────────
step "Push"

run_with_spinner "Pushing to origin" git -C "$ROOT" push --follow-tags
success "Pushed with tags"

# ── Step 7: Create GitHub Release ────────────────────────────────
step "Create GitHub Release"

if command -v gh &>/dev/null; then
  RELEASE_NOTES=""
  if command -v git-cliff &>/dev/null; then
    RELEASE_NOTES=$(git-cliff --current --strip header 2>/dev/null || true)
  fi

  if [[ -z "$RELEASE_NOTES" ]]; then
    gh release create "v${NEXT_VERSION}" \
      --title "v${NEXT_VERSION}" \
      --generate-notes
  else
    gh release create "v${NEXT_VERSION}" \
      --title "v${NEXT_VERSION}" \
      --notes "$RELEASE_NOTES"
  fi
  success "GitHub Release created"
else
  info "${YELLOW}gh CLI not found — skipping GitHub Release creation${RESET}"
  info "Install: ${DIM}https://cli.github.com${RESET}"
fi

# ── Done ──────────────────────────────────────────────────────────
echo ""
echo -e "${DIM}─────────────────────────────────────${RESET}"
echo -e "${GREEN}${BOLD}  Released v${NEXT_VERSION}${RESET}"
echo -e "${DIM}  GitHub Actions will publish to crates.io and npm.${RESET}"
echo ""
