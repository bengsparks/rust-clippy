use crate::utils::{clip, is_integer_const, span_lint_and_help};
use if_chain::if_chain;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::{self};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// **What it does:** Finds usages of is_digit that can be replaced with is_ascii_digit or is_ascii_hexdigit.
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // c.is_digit(10)
    /// // c.is_digit(16)
    /// ```
    /// Use instead:
    /// ```rust
    /// c.is_ascii_digit()
    /// c.is_ascii_hexdigit()
    /// ```
    pub IS_DIGIT_SIMPLIFY,
    style,
    "Finds usages of is_digit that can be replaced with is_ascii_digit or is_ascii_hexdigit"
}

declare_lint_pass!(IsDigitSimplify => [IS_DIGIT_SIMPLIFY]);

impl LateLintPass<'_> for IsDigitSimplify {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        if_chain! {
            // method call
            if let ExprKind::MethodCall(path, _, args, span) = &expr.kind;
            // calling is_digit
            if path.ident.name == sym!(is_digit);
            if args.len() == 2;

            // on a char instance
            if let ty::Char = cx.typeck_results().expr_ty(&args[0]).kind();

            // with an unsigned integer (parameter is u32)
            if let ty::Uint(ity) = cx.typeck_results().expr_ty(&args[1]).kind();

            then {
                let messages = if is_integer_const(cx, &args[1], clip(cx.tcx, 10, *ity)) {
                    Some(("is_digit called with decimal radix", "consider using is_ascii_digit"))
                } else if is_integer_const(cx, &args[1], clip(cx.tcx, 16, *ity)) {
                    Some(("is_digit called with hexadecimal radix", "consider using is_ascii_hexdigit"))
                } else {
                    None
                };

                if let Some((reason, sugg)) = messages {
                    span_lint_and_help(cx, IS_DIGIT_SIMPLIFY, *span, reason, None, sugg)
                }
            }
        }
    }
}
