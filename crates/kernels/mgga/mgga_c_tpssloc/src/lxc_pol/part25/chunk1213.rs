//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1213/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1213<F: Float>(t82122: F, t10104: F, t10110: F, t10111: F, t10112: F, t2053: F, t24281: F, t2718: F, t2719: F, t40890: F, t7087: F, t7092: F, t7106: F, t82113: F, t82115: F, t82120: F, t82126: F, t855: F, t865: F, t9590: F) -> F {
    let t85060 = F::cast_from(0.3244175520728446583e0_f64) * t82122;
    let t85071 = F::cast_from(0.9869604401089358619e-1_f64) * t82113 - F::new(6.0) * t7087 * t10112 + F::new(6.0) * t9590 * t7092 - F::cast_from(0.46058153871750340221e0_f64) * t82115 - F::new(18.0) * t855 * t10110 * t7106 * t2719 + F::cast_from(0.9869604401089358619e-1_f64) * t82120 - t85060 + F::new(24.0) * t855 * t40890 * t2053 * t10111 - t7087 * t10104 + F::new(6.0) * t855 * t2718 * t24281 * t865 - F::cast_from(0.49348022005446793095e-1_f64) * t82126;
    t85071
}
