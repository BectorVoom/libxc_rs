//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1162/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1162<F: Float>(t10078: F, t10082: F, t10100: F, t10104: F, t10118: F, t10131: F, t10138: F, t12970: F, t12974: F, t12978: F, t12982: F, t12986: F, t12993: F, t3271: F) -> F {
    let t12994 = -F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t10078 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t10082 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t10100 + t3271 * t12970 / F::cast_from(384.0_f64) + t3271 * t12974 / F::cast_from(768.0_f64) + t3271 * t12978 / F::cast_from(768.0_f64) - t3271 * t12982 / F::cast_from(1536.0_f64) - t3271 * t12986 / F::cast_from(3072.0_f64) - t10104 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t10118 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t10131 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t10138 + t12993;
    t12994
}
