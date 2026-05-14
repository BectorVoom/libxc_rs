//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1160/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1160<F: Float>(t23109: F, t23110: F, t232: F, t98779: F, t5587: F, t81886: F, t23041: F, t5619: F, t23133: F, t5624: F, t1516: F, t87340: F, t16673: F, t6620: F, t23083: F, t28375: F) -> (F, F, F, F, F, F, F) {
    let t98791 = t23109 * t23110 * t98779 * t232;
    let t98796 = t81886 * t5587;
    let t98798 = t23041 * t5619;
    let t98828 = t23133 * t5624;
    let t98830 = t87340 * t1516;
    let t98832 = t16673 * t6620;
    let t98836 = t23083 * t28375;
    (t98791, t98796, t98798, t98828, t98830, t98832, t98836)
}
