use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_semver::RustcVersion;
use rustc_session::{declare_tool_lint, impl_lint_pass};

use crate::utils;

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub RECOMMEND_DERIVING,
    style,
    "default lint description"
}

pub struct RecommendDeriving {
    msrv: Option<RustcVersion>,
}

impl RecommendDeriving {
    pub fn new(msrv: Option<RustcVersion>) -> Self {
        RecommendDeriving { msrv }
    }
}

impl_lint_pass!(RecommendDeriving => [RECOMMEND_DERIVING]);

impl LateLintPass<'_> for RecommendDeriving {
    fn check_impl_item(&mut self, cx: &LateContext<'_>, impl_item: &'tcx ImplItem<'_>) {
        
    }

    utils::extract_msrv_attr!(LateContext);
}
