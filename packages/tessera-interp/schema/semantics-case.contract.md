# Semantics case contract

The contract for the semantics test suite of work package C
([spec §13](../../../docs/spec/v0.1.md#13-swarm-work-packages-and-milestones)),
owned by `tessera-interp`. **This contract is provisional**: the semantics
test-suite format is not frozen yet, so the format may be reworked before its
freeze point, and pre-freeze rework is accepted — see
[Status](#status-provisional-pre-freeze-rework-accepted).

| | |
|---|---|
| Machine-readable schema | [`semantics-case.schema.json`](semantics-case.schema.json) |
| Conformance suite | [`../tests/semantics_schema.rs`](../tests/semantics_schema.rs) |
| Cases | [`../../../corpus/semantics/`](../../../corpus/semantics/) |
| Normative semantics | [spec §5](../../../docs/spec/v0.1.md#5-execution-semantics) |
| Work-start `main` commit | `40a493057d4587b4d35eb4b85560b8fd2ff3e09d` |

## Status: provisional, pre-freeze rework accepted

Spec §13 freezes interfaces in this order: **AST schema → Core IR schema →
diagnostics JSON → semantics test-suite format → Low IR schema → hash
normalization.** At the work-start commit above, none of the first three is
frozen: `tessera-syntax`, `tessera-ir` and `tessera-diag` are scaffolds with no
schema yet. The semantics test-suite format is therefore the *fourth* thing to
freeze, and this document describes it **before** that freeze.

**The semantics test-suite format is not frozen.** This contract, its schema
and the corpus cases are a *provisional* working contract, not a frozen
interface, and they are delivered ahead of the freeze point in the order above.

Three interfaces this contract depends on are named here and are explicitly
**provisional**:

| Interface | Owner | Status at the work-start SHA |
|---|---|---|
| AST schema | `tessera-syntax` (WP A) | **Provisional** — not frozen, no schema exists |
| Core IR schema | `tessera-ir` / `tessera-check` (WP B) | **Provisional** — not frozen, no schema exists |
| Diagnostics JSON | `tessera-diag` (shared) | **Provisional** — not frozen, no schema exists |

**Pre-freeze rework is accepted.** While these three interfaces are provisional,
this contract and its schema may change — in the field set, in the naming, or in
the shape of a case — without a versioned proposal PR and without a green
differential-test run, which is what the freeze order reserves for changes to
frozen interfaces. What is *not* waived: once the AST schema, Core IR schema
and diagnostics JSON are frozen, and with them the semantics test-suite format,
any later change to this contract needs a versioned proposal PR and a green
differential-test run (spec §13).

The cases in `corpus/semantics/` are the only place seeded state lives
([corpus README](../../../corpus/README.md)); v0.1 defines no source form for
seeding state, and every `example` runs against a fresh, empty store.

## What a case is

A case is one JSON document describing a complete run of one module:

```
(initial_state, inputs) -> (expected_state, expected_outputs)
```

It fixes the seeded starting state, the rule calls to make in order, the
external inputs those calls consume, the result of every call, and the state
and outputs the run must end with. The interpreter is the oracle (spec §1,
P1): a backend that disagrees with the interpreter on a case is wrong, and the
parity gate compares them on exactly these cases.

## Fields

| Field | Required | Meaning |
|---|---|---|
| `module` | yes | The module the case runs against. A name is a label, not identity (P7). |
| `initial_state` | yes | The seeded persistent state: `tables`, each a list of rows. A table absent from `tables` starts empty. |
| `calls` | yes | The rule calls, in order. At least one. |
| `expected_state` | yes | The persistent state after the last call. |
| `expected_outputs` | yes | The delivered output elements, in delivery order. May be empty. |

### `initial_state` and `expected_state`

Both have the shape `{ "tables": { "<table>": [ <row>, … ] } }`. A table is a
declared `state` table (spec §4); a row is an object whose fields are the
closed, deterministic literals of spec §2–§3, so a row is JSON-representable
and replays identically. An id literal is the tagged object
`{ "id": 12, "type": "Id<Task>" }`, so an id is distinguishable from a `Text`
that happens to look like a number.

A case never names a table that the module does not declare, and never carries
a field the module does not declare in that table. Both are checked by the
interpreter when it loads the case, not by the schema, which cannot know the
module's declarations.

### `calls`

Each call has:

| Field | Required | Meaning |
|---|---|---|
| `index` | yes | The call's position in the `calls` array, counted from 0. |
| `rule` | yes | The rule being called. |
| `args` | no | The call arguments, in source order. |
| `inputs` | no | The external inputs this call consumed, in the order they were consumed. |
| `result` | yes | Exactly one of `Ok(v)` or `Failed(Name)`. |

**Order.** The array order *is* the execution order. Rule calls on one store
commit in a total order recorded in the input log (spec §5.5), and the case
records that order. `index` MUST equal the call's position in the array; a case
where the two disagree is malformed and is rejected. JSON Schema 2020-12 has no
keyword for cross-item ordering, so this rule — and only this rule — is
enforced by the conformance suite rather than by the schema. The suite is the
reference implementation of this contract and ships with the schema, so the two
cannot drift apart silently.

**Results.** `result` is a tagged object with exactly one tag:

```json
{ "ok": <value> }          // the published success value
{ "failed": "NotFound" }   // the named failure
```

A call whose result has both tags, neither tag, or a tag the schema does not
define is malformed and is rejected. The schema enforces this with `oneOf`:
exactly one of the two tags may be present. `Failed(LawViolation)`,
`Failed(DivisionByZero)` and `Failed(RefinementViolation)` are the reserved
runtime failures of spec §5.2 and §3; a case may observe them, but they are
not user-defined failure names and a module may not declare them.

A rule that returns nothing publishes the unit value; the corpus writes that
as `{"ok": true}`.

### `inputs` — the input log

Every external input — a clock reading, a random draw, an id generation, an
external request — is appended to the input log *before* it is used
(spec §5.6). A case records that log: `inputs` lists the inputs one call
consumed, in consumption order.

| Field | Required | Meaning |
|---|---|---|
| `kind` | yes | `clock`, `random`, `id` or `request`. |
| `value` | yes | The logged value. |
| `capability` | for `clock` | The capability binding the reading came from (`clock`). |
| `reading` | for `clock` | `now` (an Instant) or `today` (the UTC calendar Date). |

A `clock` input MUST name both its capability and which reading it is: `now`
and `today` are separate readings, are separately logged, and are not required
to be simultaneous (spec §6). `today` is the proleptic-Gregorian calendar date
of the reading in UTC; no host time zone or DST rule may affect it, and a
replayed reading yields the same date.

Replaying the log reproduces the run exactly (spec §5.6). The case therefore
carries the values themselves: a backend that consults the wall clock, the
system RNG or a real id generator instead of the logged values is wrong, even
if the values happen to coincide.

### `expected_outputs`

The elements delivered to the surface, in delivery order: statement source
order within a call, then the serial commit order across calls (spec §5,
*Output*). Each element names the declared output stream it was appended to
and its value.

## Normative semantics

These rules are the spec's, restated for the case format. The spec is
authoritative; the case format only says how a case records them.

**Atomic rules (spec §5.1).** A rule call reads one snapshot, computes all
changes, checks the laws that mention touched state against the proposed
post-state, and commits or aborts as a whole. On abort: all proposed writes and
all buffered outputs are discarded, and no success value is published. So for a
call that ends in `Failed(Name)`, the state after the call equals the state
before it, and the outputs buffered by that call do not appear in
`expected_outputs`.

**Failures are values (spec §5.2).** A caller sees `Ok(v)` or `Failed(Name)`.
Every failure obeys the same atomic abort and input-log retention rules.

**Input-log retention (spec §5.1).** Inputs already appended to the log are
retained, *including inputs consumed by an aborted call*. So the `inputs` of a
failed call are still recorded in the case, and the run remains replayable. A
case whose failed call lists no inputs, when the call consumes any, is
incomplete — but the schema cannot see that, because "which inputs a rule
consumes" is a property of the rule, not of the case document. The
interpreter checks it when it replays the case.

**Serial commits (spec §5.5).** Calls on one store commit in a total order.
The `calls` array is that order.

**Outputs are delivered only after commit (spec §5, *Output*).** The surface
delivers buffered elements only after a successful commit; an abort discards
every buffered element along with the proposed state writes, and the input log
is unchanged by that discard.

**Determinism (spec §1, P1).** Same case, same result, on every backend, bit
for bit. The conformance suite asserts that a case reads back identically and
validates identically on a second read: no map iteration, no clock and no
randomness may reach the result.

## The conformance suite

[`../tests/semantics_schema.rs`](../tests/semantics_schema.rs) is the suite
that ships with this contract. It:

1. validates every worked case against the schema and against the ordering
   rule;
2. asserts that `corpus/semantics/` holds exactly the three worked cases;
3. asserts that reading a case twice yields identical bytes and an identical
   verdict;
4. asserts that the three cases collectively cover an `Ok(v)` result, a
   `Failed(Name)` result, and logged `clock`, `random` and `id` inputs;
5. rejects, one rule at a time, a case with a missing required field, a
   mistagged result, a misordered call sequence, a malformed input log, and a
   malformed state or output.

Run it with:

```
cargo test -p tessera-interp --test semantics_schema
```

## The worked cases

| Case | Demonstrates |
|---|---|
| [`successful-commit.json`](../../../corpus/semantics/successful-commit.json) | Seeded initial state, one ordered call, a logged clock reading, an `Ok(v)` result, the committed final state and one delivered output. |
| [`failed-call.json`](../../../corpus/semantics/failed-call.json) | A `Failed(Name)` result: the state is unchanged, the buffered output is discarded, and the inputs consumed by the aborted call stay in the log. |
| [`nondeterministic-inputs.json`](../../../corpus/semantics/nondeterministic-inputs.json) | Two ordered calls consuming logged `clock`, `random` and `id` inputs, with the deterministic expected state and the ordered expected outputs. |

## What this contract does not decide

- **Which tables a module declares.** The schema cannot see a module's
  declarations; the interpreter checks a case against them when it loads it.
- **Which inputs a rule consumes.** The schema records the log; the rule
  decides what it reads. The interpreter checks the two agree on replay.
- **How a value is typed.** The schema requires closed, JSON-representable
  values; the Core IR schema (provisional) owns their types.
- **The diagnostics JSON.** A malformed case is rejected by the conformance
  suite; the code it reports is the diagnostics JSON's business, and that
  interface is provisional too.
