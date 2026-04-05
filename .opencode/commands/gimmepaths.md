---
description: Print out important paths to continue the work in the other llm
agent: build
model: zai-coding-plan/glm-4.5-air
---

Before terminating due to context limits, output all relative file paths necessary to resume the current task exactly where we left off.

Include these categories (omit entire section if empty):
- Important paths: important files that you would need to read to continue the work after the context wipeout.
- Context: conversation summary, or notes on current objectives/blockers

Format for each path:
`./relative/path/to/file.ext` - [One-line description: current content status or specific next step required]

Rules:
- Use relative paths from current working directory  
- Only include files essential for task continuation
