//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1027/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1027<F: Float>(t23602: F, t3127: F, t1011: F, t3131: F, t225: F, t23592: F, t11094: F, t1958: F, t2752: F, t28: F, t111: F, t2022: F, t192: F, t531: F, t1982: F, t7450: F) -> (F, F, F, F, F, F, F, F) {
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    let t23696 = t23592 * t225;
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    let t23880 = t2022 * t111;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t24999 = t7450 * t111;
    (t23677, t23678, t23696, t23742, t23788, t23880, t24995, t24999)
}
