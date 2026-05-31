//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1039/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1039<F: Float>(t12890: F, t185: F, t2250: F, t4195: F, t4194: F, t4303: F, t870: F, t262: F, t4119: F, t2553: F, t4315: F, t9717: F) -> (F, F, F, F, F, F) {
    let t12891 = t12890 * t185;
    let t12892 = t4195 * t2250;
    let t12894 = F::cast_from(12.0_f64) * t4194 * t12892;
    let t12895 = t4303 * t870;
    let t12899 = t262 * t4119;
    let t12903 = t4315 * t2553;
    let t12906 = F::cast_from(0.5848223622634646207e0_f64) * t9717;
    (t12891, t12894, t12895, t12899, t12903, t12906)
}
