# ARCHITECTURE.md — a-townchain-os-docs
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
a-townchain-os-docs/
├── package.json               # Package configuration for documentation site
├── tsconfig.json             # TypeScript configuration
├── README.md                 # Documentation portal overview
├── REALITY_STATUS.md         # Comprehensive reality status checklist
├── KONSOLIDIERUNGS_MATRIX.md # Consolidation matrix for ecosystem modules
├── KONSOLIDIERUNGS_ROADMAP.md# Ecosystem migration and consolidation roadmap
├── docs/                     # Full technical documentation and standard specs
├── src/                      # Documentation site UI components and renderer
│   ├── components/           # Navigation, markdown renderers, and search
│   └── pages/                # Documentation site routes
└── TODO/                     # Master task lists and documentation backlog
```

## Module Descriptions
- package.json — Build configuration and site dependencies
- tsconfig.json — TypeScript options for doc site renderer
- README.md — Entry point explaining documentation structure
- REALITY_STATUS.md — Consolidated status of all ATC repositories and standards
- KONSOLIDIERUNGS_MATRIX.md — System-wide consolidation mapping across repos
- KONSOLIDIERUNGS_ROADMAP.md — Timeline and milestones for monorepo consolidation
- docs/ — Markdown documentation tree covering architecture, protocols, and standards
- src/components/ — React / TypeScript components for interactive doc navigation
- src/pages/ — Page templates and routing structure
- TODO/ — Implementation backlog and task registers

## Build System
- npm / Next.js / Static Site Generator

## Dependencies
- Node.js, TypeScript, React, Next.js

## Status (Active/Migrated/Legacy)
Active (TypeScript)
