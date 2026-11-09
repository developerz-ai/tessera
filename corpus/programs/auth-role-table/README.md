# auth-role-table

`table.tes` — a `state users: Table<User> key id;` (keyed, like every other
program's tables) holding record-valued fields (`role`, `name`), a rule that
binds the row it writes (`let target = users[u] or fail NotFound;`), writes
through that binding (`users[target.id].role = role;`) so no local is left
dangling and the row read is the row written, returns the value it just assigned
(`return role;` at :18), and carries a law reading the same field, a stream append
of a record literal (`output ledger += { user: u, granted: role };` — v0.1 has
no standalone local record-typing, so the append projects the `Entry` shape
inline), and a two-fixture-argument `example` (`grant(bob, admin_role)` with
both arguments declared fixtures). All shapes
ride existing anchors — no fresh constructs; the `#15` anchor commits the
two-arg fixture form.

## Issue anchors

| Issue | Used? |
|---|---|
| #5 keyword list | YES — `return` at statement position (`:18`) |
| #6 keywords without productions | NONE |
| #7 statement separator | YES — `;` terminates every statement, including the `or fail` forms at :15 |
| #8 G4-fragment productions | NONE |
| #9 parameterized capability grammar | YES — `needs` clause (`:9`) |
| #10 literal collisions / id-literal typing | YES — ascribed id literal :11 |
| #15 example fixture origin | YES — fixture-only example args (`bob, admin_role` at :23) |
