//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1125/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1125<F: Float>(t1014: F, t82514: F, t3032: F, t360: F, t1009: F, t343: F, t25490: F, t225: F, t82390: F, t3158: F, t6796: F, t23600: F, t995: F, t10336: F, t1920: F, t1949: F) -> (F, F, F, F, F, F, F) {
    let t82637 = t82514 * t1014;
    let t82638 = t3032 * t360;
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    let t82676 = t82390 * t225;
    let t82716 = t6796 * t3158;
    let t82736 = t23600 * t995;
    let t82799 = 0.30461741978670859935e-2 * t1920 * t10336 * t1949;
    (t82637, t82638, t82655, t82676, t82716, t82736, t82799)
}
