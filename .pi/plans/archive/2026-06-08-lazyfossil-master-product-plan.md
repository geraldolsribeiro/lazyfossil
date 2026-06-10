---
title: "Lazyfossil master product plan"
status: draft
created: "2026-06-08T17:17:45.417Z"
type: feature
---

## Goal
Make lazyfossil the canonical terminal UI for Fossil SCM: a Fossil-first daily driver that feels native, reliable, and low-friction.

## Product Promise
When someone asks for a Fossil TUI, the answer should be lazyfossil.

## Principles
- Fossil-first, not Git-with-new-words
- Fast to scan, fast to act
- State should be obvious; noise should stay low
- Minimal configuration; sensible defaults
- Reliability before novelty
- Documentation, demos, and packaging are part of the product

## Strategic Goals
### 1. Embrace Fossil-specific workflows
Focus on what makes Fossil different:
- Timeline with branch/merge context
- Tags and branches
- Check-ins and artifact inspection
- Repo sync status and unpublished local check-ins
- Eventually: tickets, wiki, forum, technotes

### 2. Become the recommended Fossil TUI
Make lazyfossil the default answer by:
- Keeping install and setup friction extremely low
- Shipping binaries for Linux, macOS, and Windows
- Maintaining excellent docs, screenshots, and demos
- Publishing clear release notes and examples

### 3. Work closely with Fossil users and developers
Build credibility by:
- Engaging on the Fossil forum and release threads
- Soliciting design feedback from core developers
- Shipping polished, trustworthy releases

### 4. Make timeline a first-class feature
The timeline should be a signature capability:
- Interactive navigation
- Timeline filtering
- Jump from entry to diff/details
- Visual branch/merge graph context

### 5. Expose Fossil’s strengths
Surface features users may miss:
- Autosync
- Private branches
- Unversioned content
- Tags
- Bisect
- Technotes

### 6. Improve discoverability
Help users find and understand the app:
- Short demos/GIFs
- Release screenshots
- Clear onboarding copy
- Contextual hints and help

### 7. Keep configuration minimal
Prefer:
- Zero-config defaults
- Few global settings
- No theme maze
- No excessive customization before product value

### 8. Build reliability before features
Prioritize:
- Correctness
- Stability
- Performance
- Predictable behavior in real checkouts

### 9. Integrate external tools well
Support power-user workflows with:
- External diff tools
- External editor/open actions
- Seamless handoff to user-preferred tooling

### 10. Add repository health views
Show repo-centric status clearly:
- Modified/added/deleted/missing/conflicted counts
- Incoming/outgoing changes
- Autosync state
- Unpublished local check-ins

### 11. Think beyond source control
Long-term, evolve into a repository cockpit:
- Read-only ticket browsing
- Wiki browsing
- Forum reading
- Technote viewing

## Priorities
### Near-term
1. Timeline improvements
2. Branch and tag management
3. Better diff navigation
4. External diff/editor integration
5. Contextual help

### Mid-term
6. Repository health dashboard
7. Stronger sync/status visibility
8. Discovery and packaging polish
9. Read-only ecosystem views

## Non-goals
- Recreating lazygit
- Git-centric staging metaphors
- Large configuration surfaces
- Cluttered panels that hide the current task

## Success Criteria
- Fossil users recommend it to each other
- Common tasks feel faster than the CLI alone
- New users can understand the app without Fossil expertise
- The app feels like Fossil’s natural terminal companion
