---
name: search-icons
description: Search and validate icon names from Iconify before using them in Icon components
---

## What I do

Verify that icon names actually exist in the Iconify collection before adding them to code. Prevents invisible/missing icons caused by typos or wrong icon names.

## When to use me

Use this when you need to add an `<Icon name="..." />` component and are unsure of the exact icon name.

## How to search

### Step 1: Browse collections index (local)

```bash
# List all available collections
jq 'keys' node_modules/@iconify/collections/collections.json

# Get metadata and sample icon names for a collection
jq '.["{collection}"]' node_modules/@iconify/collections/collections.json
```

### Step 2: Validate candidate icon names (API)

```bash
# Check if a single icon exists in a collection
curl -s "https://api.iconify.design/{collection}.json?icons={icon-name}&width=24" | jq '.icons | keys'
# Empty result + .not_found means the icon does NOT exist

# Check multiple candidates at once (comma-separated)
curl -s "https://api.iconify.design/{collection}.json?icons=icon-a,icon-b,icon-c&width=24" | jq '.'
```

### Step 3: Search by keyword (API)

```bash
# List sample icons from a collection
curl -s "https://api.iconify.design/collections?prefix={collection}" | jq '.["{collection}"].samples'
```

## Rules

1. NEVER guess icon names — always verify before writing them
2. If the requested icon name doesn't exist, search for alternatives and present options to the user
3. Return the confirmed valid icon name(s) to the calling agent
4. The most common collections used in this project are `prime` (PrimeIcons)

## Nuxt Icon (`@nuxt/icon`) reference

The project uses `@nuxt/icon` which wraps Iconify. Key facts:

- **Module**: `@nuxt/icon` — registered in `nuxt.config.ts` under `modules`
- **Component**: `<Icon name="..." />` (auto-imported, no explicit import needed)
- **Props**: `name` (required), `size` (default `1em`), `mode` (`svg` or `css`, default `css`)
- **Icon name format**: `{collection}:{icon-name}` — e.g. `<Icon name="prime:heart" />`

### Common patterns

- **Conditional icon** (scan-friendly):

  ```vue
  <!-- Good — scanner can detect both names -->
  <Icon :name="dark ? 'prime:moon' : 'prime:sun'" />

  <!-- Bad — scanner cannot detect dynamic names -->
  <Icon :name="`prime:${dark ? 'moon' : 'sun'}`" />
  ```

- **Custom SVG collection**: place `.svg` files in a directory, register via `icon.customCollections` in `nuxt.config.ts`
- **TailwindCSS v4 + css mode**: set `cssLayer: 'base'` in `app.config.ts` under `icon`
