# Require Lifetimes

Require Lifetimes is a crate which provides the `#[require_lifetimes]` annotation.
This annotation ensures that the annotated function *must* have lifetimes for
every reference in its signature. In other words, it forces the author to
not rely on lifetime elision for a given function. It also stops the author
from using the anonymous lifetime `'_`.

The intent behind this crate is to force the user to actually understand lifetimes.
This will be useful when teaching students how lifetimes work, or when trying to
write explanatory material. This crate should not be used in production applications,
as adding unnecessary lifetimes is an anti-pattern.

## Examples

See the `ui_tests` folder for examples of programs with this annotation.
Files ending in `_err.rs_` have a corresponding `.stderr` that shows what
their output will be.
