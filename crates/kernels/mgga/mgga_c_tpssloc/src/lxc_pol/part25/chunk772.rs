//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 772/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk772<F: Float>(t2862: F, t931: F, t932: F, t2904: F, t938: F, t10524: F, t951: F, t10603: F, t10629: F, t315: F, t10632: F, t2853: F, t923: F, t2885: F, t919: F, t10717: F, t10720: F, t10724: F, t10729: F, t10733: F, t10734: F, t10739: F, t10740: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2905: F, t2907: F, t2930: F, t933: F, t943: F) -> (F, F) {
    let t10743 = t2862 * t931;
    let t10744 = t10743 * t932;
    let t10747 = t938 * t2904;
    let t10750 = t10524 * t951;
    let t10753 = t10603 * t951;
    let t10756 = t315 * t10629;
    let t10757 = t10524 * t10632;
    let t10760 = t2853 * t923;
    let t10765 = t919 * t2885;
    let t10768 = 0.96491876992155210402e2 * t2886 * t10717 - 0.35089341735807877242e1 * t2905 * t10720 + 0.51947577317044391277e2 * t2930 * t10724 + t10729 - t10733 - 6.0 * t2861 * t10734 - t10739 - 6.0 * t10740 * t2863 + 6.0 * t2886 * t10744 - 0.35089341735807877242e1 * t10747 * t2907 + 0.35089341735807877242e1 * t2930 * t10750 + 0.5848223622634646207e0 * t943 * t10753 + 0.10254018858216406658e4 * t10756 * t10757 + 3.0 * t10760 * t933 + 3.0 * t2856 * t2881 + 0.96491876992155210402e2 * t10765 * t2889;
    (t10743, t10768)
}
