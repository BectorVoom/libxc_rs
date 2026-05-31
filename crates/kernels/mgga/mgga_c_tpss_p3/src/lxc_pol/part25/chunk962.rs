//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 962/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk962<F: Float>(t1170: F, t4430: F, t1173: F, t4377: F, t724: F, t489: F, t2215: F, t4438: F, t2206: F, t10039: F, t3240: F, t4409: F) -> (F, F, F, F, F, F, F) {
    let t12913 = F::cast_from(8.0_f64) * t1170 * t4430;
    let t12915 = F::cast_from(8.0_f64) * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = F::cast_from(2.0_f64) * t489 * t12916;
    let t12920 = t4438 * t2215;
    let t12922 = t4438 * t2206;
    let t12924 = F::cast_from(4.0_f64) * t10039;
    let t12993 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3240 * t4409;
    (t12913, t12915, t12918, t12920, t12922, t12924, t12993)
}
