//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 806/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk806<F: Float>(t1824: F, t7722: F, t214: F, t6434: F, t225: F, t28108: F, t1808: F, t254: F, t1835: F, t28053: F, t10143: F, t1408: F, t5631: F, t28437: F, t28442: F, t1520: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t97189 = t7722 * t1824;
    let t97511 = t214 * t6434;
    let t97558 = t28108 * t225;
    let t97626 = t1808 * t254;
    let t97740 = t1835 * t254;
    let t97756 = t28053 * t225;
    let t98064 = t10143 * t1408;
    let t98133 = t214 * t5631;
    let t98166 = t28437 * t225;
    let t98239 = t28442 * t225;
    let t98279 = t1520 * t254;
    (t97189, t97511, t97558, t97626, t97740, t97756, t98064, t98133, t98166, t98239, t98279)
}
