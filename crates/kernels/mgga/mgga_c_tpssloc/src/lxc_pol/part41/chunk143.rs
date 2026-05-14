//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 143/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk143<F: Float>(t382: F, t388: F, t193: F, t293: F, t328: F, t330: F, t336: F, t265: F) -> (F, F) {
    let t390 = t382 * t388 + 1.0;
    let t391 = f64::ln(t390);
    let t394 = t193 * t336 * t391 - t293 + t328 + t330;
    let t395 = t265 < t394;
    let t396 = piecewise3(t395, t394, t265);
    (t390, t396)
}
