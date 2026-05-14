//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 550/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk550<F: Float>(t2684: F, t860: F, t235: F, t2710: F, t226: F, t255: F, t2613: F, t2617: F, t2729: F, t2733: F, t2736: F, t808: F, t812: F, t861: F, t863: F, t858: F) -> (F, F, F, F) {
    let t2738 = t860 * t2684;
    let t2740 = t235 * t2710;
    let t2742 = t226 * t2740 + t255 * t2613 - 2.0 * t2617 * t861 + 2.0 * t2729 * t812 - 2.0 * t2733 * t812 - t2736 * t812 - t2738 * t812 + 2.0 * t808 * t863;
    let t2743 = t858 * t2742;
    (t2738, t2740, t2742, t2743)
}
