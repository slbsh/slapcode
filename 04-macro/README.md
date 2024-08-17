# Macro Expansion Extravaganza
This is a problem I've encountered while working on Shard (took me like 3 days to figure out).
I've provided a simplified version for you to solve.

Read `test.rs` for expected behaviour.

# Rules
- Don't Clone the `HashMap`
- You may Clone the `Vec` or individual `Tokens`, but only if you're sure you have to.
- (optional) do it without unsafe. (I used unsafe, but it might be possible to perhaps?).
