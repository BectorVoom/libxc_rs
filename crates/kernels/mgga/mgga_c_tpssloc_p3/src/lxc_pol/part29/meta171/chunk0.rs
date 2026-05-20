//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 905/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk905<F: Float>(t210: F, t214: F, t3734: F, t1314: F, t792: F, t118: F, t1307: F, t794: F, t3719: F, t116: F, t534: F, t212: F) -> (F, F, F, F, F, F, F) {
    let t3736 = t210 * t214 * t3734;
    let t3739 = t792 * t1314;
    let t3741 = t118 * t794 * t1307;
    let t3742 = t3739 * t3741;
    let t3745 = t210 * t214 * t3719;
    let t3748 = t534 * t116;
    let t3749 = t3748 * t212;
    (t3736, t3739, t3741, t3742, t3745, t3748, t3749)
}
