---
name: Python Trainer
description: A local-first desktop coding practice tool. Focused, calm, and immediately ready for work.
colors:
  bg-base: "#161616"
  bg-body: "#1a1a1a"
  bg-card: "#1f1f1f"
  bg-surface-raised: "#282828"
  border: "#333333"
  accent-blue: "#5b9bd5"
  accent-blue-bg: "#1a2535"
  text-primary: "#e0e0e0"
  text-secondary: "#999999"
  text-muted: "#555555"
  text-disabled: "#3a3a3a"
  accent-success: "#5a9a5a"
  accent-success-bg: "#1a2a1a"
  accent-warning: "#c07a3a"
  accent-warning-bg: "#2a2218"
  accent-error: "#9a5a5a"
  accent-error-bg: "#2a1a1a"
typography:
  body:
    fontFamily: "'Inter', system-ui, -apple-system, sans-serif"
    fontSize: "0.9rem"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "normal"
  label:
    fontFamily: "'Inter', system-ui, -apple-system, sans-serif"
    fontSize: "0.8rem"
    fontWeight: 500
    lineHeight: 1.4
    letterSpacing: "0.01em"
  mono:
    fontFamily: "'Fira Code', 'Cascadia Code', Consolas, monospace"
    fontSize: "0.85rem"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "normal"
rounded:
  xs: "4px"
  sm: "7px"
  md: "10px"
  lg: "14px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "20px"
  2xl: "24px"
components:
  button-submit:
    backgroundColor: "{colors.accent-blue}"
    textColor: "#ffffff"
    rounded: "{rounded.sm}"
    padding: "6px 14px"
  button-run:
    backgroundColor: "{colors.bg-card}"
    borderColor: "{colors.border}"
    textColor: "{colors.text-secondary}"
    rounded: "{rounded.sm}"
    padding: "6px 14px"
  filter-chip:
    backgroundColor: "{colors.bg-card}"
    textColor: "{colors.text-secondary}"
    rounded: "{rounded.sm}"
    padding: "6px 14px"
  filter-chip-active:
    backgroundColor: "{colors.accent-blue-bg}"
    textColor: "{colors.accent-blue}"
  card:
    backgroundColor: "{colors.bg-card}"
    rounded: "{rounded.md}"
    border: "1px solid {colors.bg-surface-raised}"
    padding: "18px 20px"
  nav-item-active:
    backgroundColor: "#222"
    textColor: "{colors.text-primary}"
    rounded: "{rounded.sm}"
    padding: "7px 8px"
---

# Design System: Code Trainer

## 1. Overview

**Creative North Star: "The Practice Range"**

A focused coding training ground. Clear goals, immediate feedback, visible progress, and small moments of reward. Built for practice, not entertainment. Built for consistency, not distraction.

This system is a professional-grade coding environment that stays out of the way. The palette is a desaturated dark-gray — with a single purposeful blue accent color that surfaces only when something has been done or needs doing. Typography is functional and readable for long sessions. Components are flat, tight, and undecorated. Motion exists at the edges of feedback loops, not as ambient ornamentation.

The system explicitly rejects the LeetCode and Codeforces approach of dense cluttered tables and dated UI. It rejects Duolingo-style gamification: no XP counters, no streak flames, no achievement popups demanding attention. It rejects SaaS dashboard patterns: no hero metrics, no gradient text, no marketing-page layouts, no drop-shadows. When a user opens this app, they should feel like they sat down at a well-configured workstation — ready to code in under five seconds.

**Key Characteristics:**
- Desaturated dark-gray body with one blue accent, used sparingly
- Two-pane workspace: problem description left, editor right — always
- Dual-state Daily Range page: checklist target tracking or workspace coding
- Immediate judge feedback with semantic color (success green, failure red) without noise
- Presence indicators are quiet dots, not animated banners
- Flat-by-default surfaces (no drop shadows, 1px border lines only)

## 2. Colors

A desaturated near-black palette built for long focus sessions, with one blue accent that earns its appearance.

### Primary
- **Accent Blue** (`#5b9bd5`): The single accent. Used on active nav items, selected problem borders, primary action buttons, active filter chips. Its rarity signals "this is where your attention should be." Never used decoratively or at full saturation on large surfaces.
- **Accent Blue Background** (`#1a2535` / token `accent-blue-bg`): The background wash of active nav states. Keeps the accent present without screaming.

### Neutral
- **Base Background** (`#161616`): Used for sidebar, top titlebar, and header backgrounds.
- **Body Background** (`#1a1a1a`): The page background.
- **Card Surface** (`#1f1f1f`): Slightly lifted surface for cards and panels.
- **Raised Surface** (`#282828`): Tonal background elements, card borders.
- **Structural Border** (`#333333`): All dividers, card borders, input strokes. One value, used consistently.
- **Primary Text** (`#e0e0e0`): Body copy, problem descriptions, labels. Not pure white — warm enough for long sessions.
- **Secondary Text** (`#999999`): Nav text, status descriptions, settings option labels.
- **Muted Text** (`#555555`): Helper labels, category tags, placeholder text, timestamps.
- **Disabled Text** (`#3a3a3a`): Inactive elements.

### Secondary
- **Success Green** (`#5a9a5a` / bg `#1a2a1a`): Accepted status, online presence dot, progress bar fill. Communicates success.
- **Warning Orange** (`#c07a3a` / bg `#2a2218`): Partial challenge status, score progress bar.
- **Error Red** (`#9a5a5a` / bg `#2a1a1a`): Wrong Answer status, runtime errors, failed test cases. Used only as a state signal.

### Named Rules
**The One Accent Rule.** Accent Blue appears on fewer than 10% of any given screen surface. Buttons, one active state, one selected border. Its scarcity is what makes it meaningful. Never apply it as a text gradient, background wash, or glow effect.

**The Semantic Color Rule.** Green, orange, and red are reserved for judge feedback states and target progress exclusively. They must not appear on decorative elements, categories, tags, or UI chrome. Status is never communicated by color alone — always pair with an icon, label, or text.

## 3. Typography

**Primary Font:** Inter (system-ui, -apple-system, sans-serif)
**Mono Font:** Fira Code (Cascadia Code, Consolas, monospace)

**Character:** Inter is chosen for its neutrality and legibility at small sizes — a true tool font. Fira Code brings ligatures and monospaced legibility for code. No display font is needed because the product hierarchy is about problems and code, not headlines.

### Hierarchy
- **Title** (500, 18px, 1.3): Page headers. One per screen.
- **Body** (400, 13px, 1.6): Problem descriptions, spec text, result messages. Max line length 65ch in the details column.
- **Label** (500, 12px, 1.4): Sidebar navigation buttons, setting rows, section headers.
- **Caption** (400, 11px, 1.4): Category tags, presence names, placeholder text, help text.
- **Mono** (400, 12px, 1.6): Code output, `<pre>` blocks, test case input/output comparison. Fira Code with standard tracking.

### Named Rules
**The No-Display Rule.** There are no hero headlines. No clamp-scaled display text. The largest element on screen is a page title at 18px. Hierarchy is expressed through weight and color role, not scale.

## 4. Elevation

This system is flat by default. Depth is expressed through tonal layering — `bg-base` under `bg-body` under `bg-card` — rather than shadows. The single exception is the modal overlay, which uses a `box-shadow: 0 10px 30px rgba(0,0,0,0.5)` to lift it above the page surface.

No drop-shadows on navigation items, list elements, cards, or buttons.

### Shadow Vocabulary
- **Modal Lift** (`box-shadow: 0 10px 30px rgba(0,0,0,0.5)`): Applied only to modal content containers. Signals that this surface is above the resting page.

### Named Rules
**The Flat-By-Default Rule.** At rest, all surfaces are flat. Depth is earned by modal context only. A shadow on a card or nav item is prohibited.

## 5. Components

### Buttons
- **Shape:** Gently rounded (7px). Not pill-shaped, not sharp-cornered.
- **Submit (Primary):** Accent Blue background (`#5b9bd5`) with white text. Padding 6px 14px. No border.
  - **Hover:** Darkened (`#4a8ac4`).
  - **Disabled:** Reduced opacity.
- **Run (Secondary):** Card background (`#1f1f1f`), border `1px solid #333`, secondary text.
  - **Hover:** Raised surface background (`#282828`).

### Navigation (Sidebar)
- **Container:** 200px wide, Base Background (`#161616`), right border.
- **Nav button resting:** Transparent background, 7px 8px padding, 7px radius, secondary text color.
- **Nav button active:** Darkened background fill (`#222`), primary text color, 500 weight.
- **User status card:** Bottom panel (`.sb-user`) showing current active user initials avatar, status indicator online dot, and chevron toggle.

### Checklist Rows
- **Resting:** Card background (`#1f1f1f`), 13px 16px padding.
- **Hover:** Raised surface background (`#282828`) when not done.
- **Completed:** Done state uses `opacity: 0.38` and a success green checkmark.

### Judge Result Panel
The output panel below the editor. Shows submission verdict and per-test-case results.
- **Summary bar:** Pass/Fail badge, scoring details.
- **Test case item:** Left border 3px colored (green pass / red fail). Card background fill.
- **Diff box:** Two-column, `#141414` background, 10px 12px padding. Mono font.

## 6. Do's and Don'ts

### Do:
- **Do** use `#5b9bd5` (Accent Blue) only for active interactive states. Its scarcity is the signal.
- **Do** use Success Green (`#5a9a5a`) and Error Red (`#9a5a5a`) exclusively for judge feedback states and completion checkboxes.
- **Do** pair every color-based status with a text or icon label. A colored dot alone is insufficient.
- **Do** use `1px solid var(--border)` as the single border vocabulary.
- **Do** keep the two-column workspace layout (description | editor) as the fixed application shell.
- **Do** apply `outline: 2px solid #5b9bd5; outline-offset: 2px` on focus for keyboard accessibility.

### Don't:
- **Don't** add XP counters, streak flames, level badges, achievement popups, or any gamification element.
- **Don't** use gradient text. Solid color only.
- **Don't** use glassmorphism. No `backdrop-filter: blur()` on cards or panels.
- **Don't** add hero metrics.
- **Don't** use shadows on sidebar nav items, problem list items, or cards. Flat by default.
- **Don't** use `border-left` greater than 1px as a colored stripe, except for the judge test case list (exactly 3px left border).
- **Don't** make rounded corners larger than 10px on any card or container.
- **Don't** treat this as a landing page.
