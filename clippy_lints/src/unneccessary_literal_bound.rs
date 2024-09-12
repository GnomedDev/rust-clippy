use clippy_utils::diagnostics::span_lint;
use rustc_hir::intravisit::FnKind;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyCtxt;
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
    pub UNNECCESSARY_LITERAL_BOUND,
    pedantic,
    "default lint description"
}

declare_lint_pass!(UnneccessaryLiteralBound => [UNNECCESSARY_LITERAL_BOUND]);

fn extract_anonymous_ref<'tcx>(hir_ty: &Ty<'tcx>) -> Option<&'tcx Ty<'tcx>> {
    let TyKind::Ref(lifetime, MutTy { ty, mutbl }) = hir_ty.kind else {
        return None;
    };

    if !lifetime.is_anonymous() || !matches!(mutbl, Mutability::Not) {
        return None;
    }

    Some(ty)
}


fn check_block_returns_literal(block: &Block<'_>) -> bool {
    block.stmts.iter().all(check_stmt_returns_literal) && block.expr.is_none_or(check_all_returns_literal)
}

fn check_stmt_returns_literal(stmt: &Stmt<'_>) -> bool {
    match stmt.kind {
        StmtKind::Let(LetStmt { init, els, .. }) => init.is_none_or(|init| check_all_returns_literal(init)) && els.is_none_or(check_block_returns_literal),
        StmtKind::Expr(expr) | StmtKind::Semi(expr) => check_all_returns_literal(expr),
        StmtKind::Item(_) => true,
    }
}

/// Returns `true` if all returns found were literal.
fn check_all_returns_literal(expr: &Expr<'_>) -> bool {
    // If an ExprKind isn't a return and doesn't contain any internal expressions, return true
    // as the only time we return false is for `return not_a_literal` or any other return-likes.
    match expr.kind {
        ExprKind::Lit(literal) => true,
        ExprKind::ConstBlock(_) => true,
        ExprKind::Array(init_exprs) => init_exprs.iter().all(check_all_returns_literal),
        ExprKind::Call(func_expr, arg_exprs) => arg_exprs.iter().chain([func_expr]).all(check_all_returns_literal),
        ExprKind::MethodCall(_, recv_expr, arg_exprs, _) => arg_exprs.iter().chain([recv_expr]).all(check_all_returns_literal),
        ExprKind::Tup(init_exprs) => init_exprs.iter().all(check_all_returns_literal),
        ExprKind::Binary(_, op1, op2) => check_all_returns_literal(op1) && check_all_returns_literal(op2),
        ExprKind::Unary(_, op) => check_all_returns_literal(op),
        ExprKind::Cast(op, _) => check_all_returns_literal(op),
        ExprKind::Type(op1, _) => check_all_returns_literal(op1),
        ExprKind::DropTemps(op1) => check_all_returns_literal(op1),
        ExprKind::Let(LetExpr { init, ..}) => check_all_returns_literal(init),
        ExprKind::If(cond, true_expr, false_expr) => [cond, true_expr].into_iter().chain(false_expr).all(check_all_returns_literal),
        ExprKind::Loop(block, _, _, _) => false,
        ExprKind::Match(&'hir Expr<'hir>, &'hir [Arm<'hir>], MatchSource) => false,
        ExprKind::Closure(&'hir Closure<'hir>) => false,
        ExprKind::Block(&'hir Block<'hir>, Option<Label>) => false,
        ExprKind::Assign(&'hir Expr<'hir>, &'hir Expr<'hir>, Span) => false,
        ExprKind::AssignOp(BinOp, &'hir Expr<'hir>, &'hir Expr<'hir>) => false,
        ExprKind::Field(&'hir Expr<'hir>, Ident) => false,
        ExprKind::Index(&'hir Expr<'hir>, &'hir Expr<'hir>, Span) => false,
        ExprKind::Path(QPath<'hir>) => false,
        ExprKind::AddrOf(BorrowKind, Mutability, &'hir Expr<'hir>) => false,
        ExprKind::Break(Destination, Option<&'hir Expr<'hir>>) => false,
        ExprKind::Continue(Destination) => false,
        ExprKind::Ret(Option<&'hir Expr<'hir>>) => false,
        ExprKind::Become(&'hir Expr<'hir>) => false,
        ExprKind::InlineAsm(&'hir InlineAsm<'hir>) => false,
        ExprKind::OffsetOf(&'hir Ty<'hir>, &'hir [Ident]) => false,
        ExprKind::Struct(&'hir QPath<'hir>, &'hir [ExprField<'hir>], Option<&'hir Expr<'hir>>) => false,
        ExprKind::Repeat(&'hir Expr<'hir>, ArrayLen<'hir>) => false,
        ExprKind::Yield(&'hir Expr<'hir>, YieldSource) => false,
        ExprKind::Err(ErrorGuaranteed),
    }
}

impl<'tcx> LateLintPass<'tcx> for UnneccessaryLiteralBound {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        span: Span,
        def_id: LocalDefId,
    ) {
        if span.from_expansion() {
            return;
        }

        // Checking closures would be a little silly.
        if matches!(kind, FnKind::Closure) {
            return;
        }

        // Check for `-> &str`
        let FnRetTy::Return(ret_hir_ty) = decl.output else {
            return;
        };

        let Some(inner_hir_ty) = extract_anonymous_ref(ret_hir_ty) else {
            return;
        };

        if !inner_hir_ty.is_str() {
            return;
        }

        // Check for all return statements returning literals
    }
}
