---
name: refined-design
description: Create polished, production-grade frontend interfaces with predictable design, clear hierarchy, and exceptional attention to detail. Use this skill when the user asks to build web components, pages, or applications. Generates clean, refined code that prioritizes usability and polish.
---

This skill guides creation of polished, production-grade frontend interfaces that are predictable, hierarchical, and genuinely pleasant to use. Implement real working code with exceptional attention to refinement and polish.

The user provides frontend requirements: a component, page, application, or interface to build. They may include context about the purpose, audience, or technical constraints.

## Design Thinking

Before coding, understand the context and commit to a REFINED design direction:
- **Purpose**: What problem does this interface solve? Who uses it? What are their mental models?
- **Mental models**: What patterns do users expect? How can we align with those expectations to build trust?
- **Hierarchy**: What's most important? How do we guide attention naturally through scale, color, and spacing?
- **Differentiation**: What subtle refinement makes this memorable? Excellence shows through polish, micro-interactions, and attention to detail that users notice after repeated use.

**CRITICAL**: Choose a clear, appropriate tone for the purpose and execute it with precision. Predictability creates trust; refinement creates memorable experiences.

Then implement working code (HTML/CSS/JS, React, Vue, etc.) that is:
- Production-grade and functional
- Predictable and easy to learn
- Cohesive with a clear visual hierarchy
- Meticulously refined in every detail
- Accessible and inclusive

## Frontend Aesthetics Guidelines

### Typography
System fonts are okay
- **Type system**: Define clear scale (h1-h6, body, small, caption)
- **Hierarchy**: Use size, weight, and color to create predictable information architecture
- **Readability**: Prioritize line height (1.5-1.7 for body text), letter spacing, and optimal line length (50-75 characters)

### Color & Theme
Use Tailwind CSS colors with CSS variables for consistency:
- **Tailwind palette**: Use established color categories (slate, gray, zinc, neutral, stone)
- **CSS variables**: Define semantic tokens for reuse (--color-primary, --color-text, --color-bg, etc.)
- **Accessible contrast**: Ensure WCAG AA compliance for text (4.5:1 for normal text, 3:1 for large text)
- **Semantic color usage**: Primary for CTAs, secondary for links, success/warning/danger for status

### Motion
Use meaningful animations that clarify state:
- **Purpose over surprise**: Animate to communicate, not to dazzle
- **Smooth transitions**: Use consistent timing (200-300ms) and easing (ease-out)
- **Predictable feedback**: Hover states, focus states, loading states that users anticipate
- **Subtle micro-interactions**: Button presses, checkbox toggles, form validation that feel satisfying
- **Performance**: CSS-only solutions preferred; use lightweight animation libraries for React

### Spatial Composition
Use predictable layouts with clear structure:
- **Grid systems**: Follow 8px or 4px grids for consistent spacing
- **Logical grouping**: Group related elements with proximity and borders
- **Generous whitespace**: Use whitespace to create breathing room and establish hierarchy
- **Proper alignment**: Left-align text, center-align buttons, align icons and text on baselines
- **Responsive design**: Mobile-first approach with breakpoints (640px, 768px, 1024px, 1280px)

### Visual Details
Add refinement through polish:
- **Subtle shadows**: Layered shadows for depth (e.g., `shadow-sm`, `shadow-md` in Tailwind)
- **Consistent borders**: Uniform border widths and radii
- **Smooth gradients**: Use gradients purposefully, not decoratively
- **Corner radius**: Consistent rounded corners
- **Texture**: Subtle noise or patterns only when they enhance readability or usability

## What Makes It Memorable

Excellence through refinement:
- **Exceptional attention to detail**: Every edge case, loading state, and error state considered
- **Smooth, responsive interactions**: No lag, no jank, instant feedback
- **Thoughtful micro-interactions**: Subtle feedback that feels intentional and polished
- **Accessibility**: Screen reader support, keyboard navigation, focus management that doesn't feel like an afterthought
- **Polished edge cases**: Empty states, loading skeletons, error messages that guide users
- **Consistent behavior**: Similar elements behave similarly throughout the interface

## Implementation Principles

**Match complexity to user needs**:
- Simple interfaces need careful attention to spacing, typography, and feedback
- Complex interfaces need clear structure and progressive disclosure
- Every interaction should feel intentional and polished

**Use patterns users expect**:
- Follow platform conventions (Material Design, Apple HIG)
- Use familiar UI patterns (tabs, cards, modals, dropdowns)
- Place elements where users expect them (primary actions bottom-right on desktop, bottom on mobile)

**Test edge cases**:
- What happens with long text? Very short text?
- How does this behave on mobile? Tablet?
- What if the content is empty? Loading? Failed?
- How does this work with keyboard navigation?
- What's the focus order?

## NEVER Use These Patterns

Avoid breaking predictability:
- Inconsistent spacing or alignment
- Unexpected interactions or animations
- Unclear visual hierarchy
- Abrupt animations without context
- Inconsistent colors or fonts across the interface
- Generic "AI-generated" aesthetics without polish
- Unnecessary complexity when simple works better
- Patterns that break user expectations
