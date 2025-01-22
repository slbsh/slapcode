# Tokio? naaa we've got Detroit

implement an async runtime which spawns a future on the current thread and runs it to completion.

# Rules
- Implement a custom waker
- Don't busy loop
