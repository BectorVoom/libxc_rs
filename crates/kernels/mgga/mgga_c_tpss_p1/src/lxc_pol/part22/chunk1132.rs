//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1132/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1132<F: Float>(t1586: F, t3118: F, t3144: F, t3053: F, t9751: F, t9765: F, t1561: F, t3110: F, t1133: F, t4245: F, t12352: F, t466: F) -> (F, F, F, F, F, F) {
    let t12577 = t3118 * t1586 * t3144;
    let t12580 = t9751 * t3053;
    let t12590 = t9765 * t3053;
    let t12597 = t3110 * t1561;
    let t12600 = t1133 * t4245;
    let t12607 = t466 * t12352;
    (t12577, t12580, t12590, t12597, t12600, t12607)
}
