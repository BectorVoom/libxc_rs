//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 871/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk871<F: Float>(t1970: F, t1971: F, t333: F, t511: F, t6182: F, t352: F, t515: F, t236: F, t6144: F, t118: F, t1986: F, t209: F, t44586: F) -> (F, F, F, F) {
    let t44642 = t1970 * t1971 * t511 * t6182 * t333;
    let t44647 = t1970 * t1971 * t515 * t6182 * t352;
    let t44651 = t1970 * t1971 * t236 * t6144;
    let t44655 = t1986 * t118 * t44586 * t209;
    (t44642, t44647, t44651, t44655)
}
