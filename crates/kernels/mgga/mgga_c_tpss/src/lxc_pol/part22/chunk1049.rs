//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1049/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1049<F: Float>(t30: F, t10016: F, t10022: F, t1288: F, t9924: F, t2: F, t3217: F, t1197: F, t12700: F, t1991: F, t22: F, t3218: F, t4380: F, t4383: F, t555: F, t1497: F, t9936: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t12779 = 24.0 * t10016;
    let t12780 = 48.0 * t10022;
    let t12781 = t9924 * t1288;
    let t12784 = t3217 * t2;
    let t12794 = piecewise3(t31, 0.0, 8.0 / 27.0 * t12781 * t3218 - 8.0 / 9.0 * t12784 * t12700 - 2.0 / 9.0 * t4380 * t1991 + 4.0 / 3.0 * t1197 * t555 - 4.0 * t4383 * t22);
    let t12795 = t9936 * t1497;
    (t12779, t12780, t12794, t12795)
}
