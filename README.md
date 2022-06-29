# Rough experiment with local type inference in Rust

Ideas explored:
- Rust data structures for local type inference
    - I usually implement program-languagey code with immutable data structures, but that gets weird in Rust.
    - This is an attempt to work with a mutable stack of scopes that is ergonomic, efficient, and safe. I can't yet evaluate whether the attempt is successful.
- I experimented with combining the analysis and checking modes of local type inference (Pierce and Turner, 2000). There is a lot of duplicated logic in the classic Local Type Inference rules, and Rust is verbose enough already. So in this experiment there is just one mode, and the context propagates the expected type, which is an `Option<Type>`. I've seen this approach two synthesis/checking before, but I can't remember where.
- *and* I tried type checker error recovery a la Niko Matsakis' [Responsive Compilers](https://www.youtube.com/watch?v=N6b44kMS6OM&t=3121s). This means adding type errors to the context rather than blowing up at a type error, and using a sentinel type for erroneous expressions. 
- And I combined a request for type information (like an IDE might use on hover) with code for type-checking.

# Evaluation

ergonomics:
    - The approach combines a lot of concerns. But the complexity is isolated into a fancy context data structure. So core type checking code doesn't look much different than in a "classic" batch type checker.
    - So far, the approach looks promising to me: the code in the pattern-match inside the main loop, `main::run`, looks pretty vanilla. Emphasis on "so far", since this is a tiny toy.

perf:
- Needs benchmarking. The most ergonomic and safe API I could find for `Ctx` involves a closure for every new scope, but no cloning. Probably OK.
- The string cloning is avoidable (interning) and/or cheapable (SmolStr)

## Try it

`cargo run`

## Status

Quick throway hack. There's only one example and it doesn't exercise all code paths.Some benchmarking would probably be needed if I revisit this, which I probably won't.
