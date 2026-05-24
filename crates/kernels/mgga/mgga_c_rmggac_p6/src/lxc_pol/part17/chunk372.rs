//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 372/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk372<F: Float>(t352: F, t649: F, t27: F, t2123: F, t515: F, t302: F, t668: F, t236: F, t265: F) -> (F, F, F, F) {
    let t2146 = t649 * t352;
    let t2147 = t27 * t2146;
    let t2150 = t515 * t2123;
    let t2153 = t302 * t668;
    let t2157 = t236 * t265;
    (t2147, t2150, t2153, t2157)
}
