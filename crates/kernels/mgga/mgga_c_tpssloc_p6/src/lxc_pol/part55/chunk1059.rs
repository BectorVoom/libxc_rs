//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1059/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1059<F: Float>(t3806: F, t5248: F, t550: F, t31170: F, t553: F, t835: F, t544: F, t8467: F, t1369: F, t8466: F, t31154: F, t31157: F, t31161: F, t31163: F, t31166: F) -> (F, F, F, F, F) {
    let t31172 = t5248 * t3806 * t550;
    let t31173 = t31170 * t31172;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    let t31178 = F::new(7.0) / F::new(2304.0) * t31177;
    let t31179 = t8466 * t1369;
    let t31181 = -t31154 - F::cast_from(0.48447307312968469025e-2_f64) * t31157 - t31161 - F::cast_from(0.80745512188280781708e-3_f64) * t31163 + t31166 / F::new(1536.0) - t31173 / F::new(1536.0) - t31178 - t31179 / F::new(384.0);
    (t31172, t31175, t31176, t31178, t31181)
}
