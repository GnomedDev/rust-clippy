use rustc_hir::intravisit::FnKind;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.83.0"]
    pub UNBOUNDED_LIFETIME,
    correctness,
    "default lint description"
}

declare_lint_pass!(UnboundedLifetime => [UNBOUNDED_LIFETIME]);

impl<'tcx> LateLintPass<'tcx> for UnboundedLifetime {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        if let FnKind::Method(ident, sig) = kind
            && ident.as_str() == "test_fn_method_generic_layout_asd"
        {
            println!("{decl:#?}")
        }
    }
}
