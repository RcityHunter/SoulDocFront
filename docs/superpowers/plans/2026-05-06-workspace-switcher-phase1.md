# Workspace Switcher Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the fake workspace switcher with a real front-end workspace state that starts in personal workspace and shows no teams until team data exists.

**Architecture:** Phase 1 is front-end only. `WorkspaceState` is provided globally from `App`, the sidebar reads and updates it, and hard-coded fake teams are removed. Team create/join actions are visible as entry points but do not invent team memberships.

**Tech Stack:** Rust, Dioxus, gloo-storage, existing SoulBookFront state/context patterns.

---

### Task 1: Add Workspace State

**Files:**
- Modify: `src/state.rs`
- Test: `src/state.rs`

- [ ] Add `WorkspaceKind`, `TeamWorkspace`, and `WorkspaceState`.
- [ ] Persist the active workspace key with `LocalStorage`.
- [ ] Add tests proving default is personal and missing team selections fall back to personal.

### Task 2: Provide Global Workspace Context

**Files:**
- Modify: `src/app.rs`

- [ ] Add `WorkspaceState` to Dioxus context next to `AuthState`.

### Task 3: Replace Fake Sidebar Teams

**Files:**
- Modify: `src/components/sidebar.rs`

- [ ] Read `WorkspaceState` from context.
- [ ] Render personal workspace as selected by default.
- [ ] Render "暂无团队" when `teams` is empty.
- [ ] Remove hard-coded `SoulBook 团队`, `产品研发团队`, and `Marketing`.
- [ ] Change actions to "创建团队" and "加入团队" placeholders without mutating fake team state.

### Task 4: Verify And Commit

**Files:**
- Verify: `src/state.rs`, `src/components/sidebar.rs`, `src/app.rs`

- [ ] Run `cargo test state::tests -- --nocapture`.
- [ ] Run `cargo check -q`.
- [ ] Commit only the files touched in this phase.
