# Product Spec: Code Trainer

## Register

product

## Users

Two competitive programming students, practicing and preparing for contests together. They open the app with a clear goal: complete today's coding exercises fast and stay consistent. They sit at a desktop or laptop, in a focused coding environment (bedroom, study space), often for extended sessions. No onboarding friction expected—both users know the tool intimately.

## Product Purpose

A local-first Python training desktop application built for daily coding practice. Users write Python solutions in a built-in editor, submit against automated test cases, track daily challenge progress, and see each other's real-time status. Success means: the user opens the app, starts coding within 5 seconds, completes a challenge, and feels the momentum of consistent practice.

## Brand Personality

Focused · Calm · Efficient

The product is calm enough to sit in for long sessions, focused enough that you always know what to do next, and efficient enough that nothing slows you down. Think serious craft tool, not a gamified platform. There is subtle delight — typing feedback, satisfying test case status reports, correct/wrong submission states — but it never competes with the work.

## Core Screens

1. **Home (Dashboard)**:
   - Dynamic welcome header.
   - Grid cards: Today's progress numbers, current streak, and overall daily progress bar.
   - Real-time peer accountability card showing online/solving status.
2. **Daily Range (Dual-State Workspace)**:
   - **Checklist state**: Shows categories checklist (I/O, Loops, Algorithms, etc.) with completed target states, and a grouped grid of today's challenges.
   - **Workspace state**: Two-pane solving zone with problem switcher, specifications/examples, Monaco Editor, run/submit controls, and judge outputs (expected vs actual diff, scoring progress, console outputs).
3. **Settings**:
   - Identity switcher (NG / MR3).
   - Daily requirement adjusters (minus/plus increment controls for the 6 category targets).
   - Python executable environment configuration and manual sync triggers.

## References

- **Linear**: ultra-clean layout, dark mode, no visual noise. The precision of the sidebar navigation and the single-purpose workspace is what fits.
- **Raycast**: speed and minimalism. Everything is reachable without thinking. Dark, sharp, decisive.
- **VS Code**: a coding workspace that stays out of the way. Dense information, clean layout, familiar code editing experience.
- **Notion**: comfortable whitespace, readable at long sessions, minimal chrome.
- **Riot Client**: the aesthetic reference for color — deep desaturated gray backgrounds, high-contrast accent, clean polish.

## Anti-references

- **LeetCode / Codeforces UI**: visually cluttered, dated, dense-without-purpose. Dense is fine; noisy is not.
- **Duolingo / gamified apps**: streaks, XP bars, loud achievement popups, cartoon mascots. Motivation here comes from progress, not dopamine tricks.
- **SaaS dashboards**: hero metrics, big number + small label, gradient text, gradient cards. This is a tool, not a marketing page.
- **VSCodium / heavy IDEs**: too much surface, too many panels, overwhelming. This is single-purpose.

## Design Principles

1. **Open and code in 5 seconds.** Every interaction is optimized toward reaching the editor. No loading wizards, no modal nags, no navigation mazes.
2. **Feedback that feels earned.** Submission results have presence because the user did real work. Feedback is immediate, specific, and clear.
3. **Calm over excitement, delight over noise.** Motion and microinteractions exist to reward focus, not distract from it.
4. **Status is always visible, never intrusive.** Peer presence, daily progress, and submission state are scannable at a glance without demanding attention.
5. **Dense, not cluttered.** The editor and problem description share the screen without fighting. Information density is a feature; chaos is not.

## Accessibility & Inclusion

- WCAG 2.1 AA target
- High contrast dark theme; text must meet 4.5:1 against its background
- Full keyboard navigation throughout the app
- Visible focus states on all interactive elements
- Status never communicated by color alone (icon + label pattern alongside color)
- Reduced motion: all typing animations and submission effects must have a `prefers-reduced-motion` fallback (typically crossfade or instant state change)
