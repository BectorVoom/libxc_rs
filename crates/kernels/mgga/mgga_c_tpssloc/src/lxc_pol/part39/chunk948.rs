//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 948/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk948<F: Float>(t2250: F, t2989: F, t2775: F, t343: F, t2244: F, t2987: F, t3014: F, t2262: F, t972: F, t2960: F, t2971: F, t2970: F, t2995: F) -> (F, F, F, F, F, F, F) {
    let t10245 = t2989 * t2250;
    let t10254 = t343 * t2775;
    let t10255 = t10254 * t2244;
    let t10259 = t2987 * t3014;
    let t10263 = t2262 * t972;
    let t10267 = t2960 * t2971;
    let t10273 = t2970 * t2995;
    (t10245, t10254, t10255, t10259, t10263, t10267, t10273)
}
