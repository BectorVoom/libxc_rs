//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 125/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk125<F: Float>(t171: F, t410: F, t247: F, t250: F, t369: F, t374: F) -> (F, F) {
    let t411 = t171 * t410;
    let t416 = -F::cast_from(0.86308333333333333334e0_f64) * t247 - F::new(0.301925e0) * t250 - F::new(0.5501625e-1) * t369 - F::new(0.82785e-1) * t374;
    (t411, t416)
}
