//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 677/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk677<F: Float>(t40: F, t1268: F, t2314: F, t2363: F, t5113: F, t671: F, t9347: F, t9348: F, t9351: F, t9416: F, t195: F, t2433: F, t607: F, t2250: F, t73: F, t9258: F, t9288: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t9419 = 2.0 * t1268 * t9416 + 6.0 * t2314 * t2363 + 6.0 * t2363 * t5113 + 6.0 * t671 * t9348 + t9347 + 6.0 * t9351;
    let t9427 = 1.0 / t195 / t40;
    let t9430 = t2433 * t607;
    let t9436 = piecewise3(t146, 0.0, -8.0 / 27.0 * t9427 * t9288 + 4.0 / 3.0 * t9430 * t2250 + 4.0 / 3.0 * t73 * t9258);
    (t9419, t9436)
}
