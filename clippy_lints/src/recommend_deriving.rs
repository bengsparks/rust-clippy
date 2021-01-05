use std::iter;

use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_semver::RustcVersion;
use rustc_session::{declare_tool_lint, impl_lint_pass};
use rustc_span::def_id::DefId;

use rustc_middle::ty::TyS;
use rustc_middle::ty::TyKind;

use if_chain::if_chain;

use crate::rustc_lint::LintContext;

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

trait GetDefId {
    fn get_def_id(self: &Self, cx: &LateContext<'_>) -> Option<DefId>;
}

#[derive(PartialEq, Copy, Clone)]
enum DerivableTraits {
    PartialEq,
}

#[derive(PartialEq, Copy, Clone)]
enum ImpledTraits {
    PartialEq,
}

impl GetDefId for DerivableTraits {
    fn get_def_id(self: &Self, cx: &LateContext<'_>) -> Option<DefId> {
        match self {
            // Trait injected by #[derive(PartialEq)], (i.e. "Partial EQ").
            DerivableTraits::PartialEq => cx.tcx.lang_items().structural_peq_trait(),
        }
    }
}

impl GetDefId for ImpledTraits {
    fn get_def_id(self: &Self, cx: &LateContext<'_>) -> Option<DefId> {
        match self {
            // What is this nomenclature
            ImpledTraits::PartialEq => cx.tcx.lang_items().eq_trait(),
        }
    }
}

const COMPATIBLE_IMPL_TRAITS: [ImpledTraits; 1] = [ImpledTraits::PartialEq];

impl_lint_pass!(RecommendDeriving => [RECOMMEND_DERIVING]);

impl LateLintPass<'_> for RecommendDeriving {
    fn check_impl_item<'tcx>(&mut self, cx: &LateContext<'tcx>, impl_item: &'tcx ImplItem<'tcx>) {
        let mut result = iter::once(impl_item)
            .filter_map(|ii| find_trait(cx, ii))
            .filter_map(|tr| is_compatible_trait(cx, tr))
            .filter_map(|(ct, tr)| is_derivable_impl(cx, ct, impl_item, tr));

        if let Some((to_derive, impl_item, trait_ref)) = &result.next() {
            println!("{:#?}", trait_ref);
        }
    }
}

fn find_trait<'tcx>(cx: &LateContext<'tcx>, impl_item: &'tcx ImplItem<'tcx>) -> Option<&'tcx TraitRef<'tcx>> {
    if_chain! {
        if let Some(Node::Item(item)) = cx.tcx.hir().find(cx.tcx.hir().get_parent_node(impl_item.hir_id));
        if let ItemKind::Impl { of_trait: Some(trait_ref), .. } = &item.kind;
        then {
            return Some(trait_ref);
        }
    }

    None
}

fn is_compatible_trait<'tcx>(
    cx: &LateContext<'tcx>,
    trait_ref: &'tcx TraitRef<'tcx>,
) -> Option<(ImpledTraits, &'tcx TraitRef<'tcx>)> {
    let tref_def_id = Some(trait_ref.path.res.def_id());

    println!("{:#?}, {:#?}", ImpledTraits::PartialEq.get_def_id(cx), tref_def_id);

    COMPATIBLE_IMPL_TRAITS
        .iter()
        .find(|ct| ct.get_def_id(cx) == tref_def_id)
        .map(|tr_def_id| (*tr_def_id, trait_ref))
}

fn is_derivable_impl<'tcx>(
    cx: &LateContext<'tcx>,
    ct: ImpledTraits,
    impl_item: &'tcx ImplItem<'tcx>,
    trait_ref: &'tcx TraitRef<'tcx>,
) -> Option<(ImpledTraits, &'tcx ImplItem<'tcx>, &'tcx TraitRef<'tcx>)> {
    let (sig, body) = if let ImplItemKind::Fn(sig, body_id) = &impl_item.kind {
        (sig, cx.tcx.hir().body(*body_id))
    } else {
        return None;
    };

    if ct.can_be_derived_instead(cx, sig, body) {
        Some((ct, impl_item, trait_ref))
    } else {
        None
    }
}

trait CanBeDerivedInstead {
    fn can_be_derived_instead<'tcx, 'hir>(
        self: &Self,
        cx: &LateContext<'tcx>,
        sig: &'hir FnSig<'hir>,
        body: &'hir Body<'hir>,
    ) -> bool;
}

impl CanBeDerivedInstead for ImpledTraits {
    fn can_be_derived_instead<'tcx, 'hir>(
        self: &Self,
        cx: &LateContext<'tcx>,
        sig: &'hir FnSig<'hir>,
        body: &'hir Body<'hir>,
    ) -> bool {

        fn tys_of_members(ty: &TyS<'hir>) -> &'hir [Ty<'hir>] {
            let members = match &ty.kind() {
                TyKind::Adt(adt_def, _) => adt_def,
                _ => panic!("Unexpected type: {:#?}", ty.kind()),
            };
            println!("{:#?}", members);
            return &[];
        }

        // First parameter will always be &self
        let ty = utils::walk_ptrs_hir_ty(&sig.decl.inputs[0]);
        println!("{:#?}", ty);

        let mem_tys = tys_of_members(&ty);

        fn is_derivable_peq(
            cx: &LateContext<'tcx>,
            mem_tys: &'hir [Ty<'hir>],
            sig: &'hir FnSig<'hir>,
            body: &'hir Body<'hir>,
        ) -> bool {
            println!("{:#?}", mem_tys);
            true
        }

        match self {
            ImpledTraits::PartialEq => is_derivable_peq(cx, &mem_tys, sig, body),
        }
    }
}
