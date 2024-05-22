use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprCall;
use ruff_python_semantic::Modules;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for uses of `collections.namedtuple` in stub files.
///
/// ## Why is this bad?
/// `typing.NamedTuple` is the "typed version" of `collections.namedtuple`.
///
/// Inheriting from `typing.NamedTuple` creates a custom `tuple` subclass in
/// the same way as using the `collections.namedtuple` factory function.
/// However, using `typing.NamedTuple` allows you to provide a type annotation
/// for each field in the class. This means that type checkers will have more
/// information to work with, and will be able to analyze your code more
/// precisely.
///
/// ## Example
/// ```python
/// from collections import namedtuple
///
/// person = namedtuple("Person", ["name", "age"])
/// ```
///
/// Use instead:
/// ```python
/// from typing import NamedTuple
///
///
/// class Person(NamedTuple):
///     name: str
///     age: int
/// ```
#[violation]
pub struct NamedTupleAssignment;

impl Violation for NamedTupleAssignment {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use class-based syntax for NamedTuples")
    }

    fn fix_title(&self) -> Option<String> {
        Some(format!("Use class-based syntax for NamedTuples"))
    }
}

/// PYI028
pub(crate) fn named_tuple_assignment(checker: &mut Checker, expr: &ExprCall) {
    if !checker
        .semantic()
        .seen_module(Modules::TYPING | Modules::TYPING_EXTENSIONS)
    {
        return;
    }

    let func = expr.func.as_ref();
    if checker
        .semantic()
        .resolve_qualified_name(func)
        .is_some_and(|qualified_name| {
            matches!(
                qualified_name.segments(),
                ["typing" | "typing_extensions", "NamedTuple"] | ["NamedTuple"]
            )
        })
    {
        checker
            .diagnostics
            .push(Diagnostic::new(NamedTupleAssignment, func.range()));
    }
}
