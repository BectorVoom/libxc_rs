//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1162/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1162<F: Float>(t10078: F, t10082: F, t10100: F, t10104: F, t10118: F, t10131: F, t10138: F, t12970: F, t12974: F, t12978: F, t12982: F, t12986: F, t12993: F, t3271: F) -> F {
    let t12994 = -F::new(119.0) / F::new(6912.0) * t10078 - F::new(7.0) / F::new(2304.0) * t10082 + F::new(7.0) / F::new(4608.0) * t10100 + t3271 * t12970 / F::new(384.0) + t3271 * t12974 / F::new(768.0) + t3271 * t12978 / F::new(768.0) - t3271 * t12982 / F::new(1536.0) - t3271 * t12986 / F::new(3072.0) - t10104 - F::new(7.0) / F::new(576.0) * t10118 + F::new(7.0) / F::new(144.0) * t10131 - F::new(7.0) / F::new(48.0) * t10138 + t12993;
    t12994
}
