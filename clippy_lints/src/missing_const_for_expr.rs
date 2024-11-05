use std::ops::ControlFlow;

use clippy_utils::consts::ConstEvalCtxt;
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet_opt;
use clippy_utils::visitors::Descend;
use rustc_errors::Applicability;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, ExprKind, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why restrict this?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.84.0"]
    pub MISSING_CONST_FOR_EXPR,
    restriction,
    "not the default lint description"
}

declare_lint_pass!(MissingConstForExpr => [MISSING_CONST_FOR_EXPR]);

impl LateLintPass<'_> for MissingConstForExpr {
    fn check_fn(
        &mut self,
        cx: &LateContext<'_>,
        _: FnKind<'_>,
        _: &FnDecl<'_>,
        body: &Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        clippy_utils::visitors::for_each_expr_without_closures::<!, Descend>(body, |expr| {
            if !matches!(expr.kind, ExprKind::ConstBlock(_))
                && ConstEvalCtxt::new(cx).eval(expr).is_some()
                && let Some(snippet) = snippet_opt(cx, expr.span)
            {
                span_lint_and_sugg(
                    cx,
                    MISSING_CONST_FOR_EXPR,
                    expr.span,
                    "this expression could be inside a `const` block",
                    "replace with",
                    format!("const {{ {snippet} }}"),
                    Applicability::MachineApplicable,
                );

                ControlFlow::Continue(Descend::No)
            } else {
                ControlFlow::Continue(Descend::Yes)
            }
        });
    }
}
