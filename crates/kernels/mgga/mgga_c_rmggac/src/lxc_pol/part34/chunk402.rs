//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 402/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk402<F: Float>(t236: F, t8829: F, t333: F, t615: F, t511: F, t352: F, t515: F, t1685: F, t71: F, t131: F, t2338: F, t356: F, t2164: F, t574: F, t1656: F, t640: F) -> (F, F, F, F, F, F, F, F) {
    let t8830 = t236 * t8829;
    let t8834 = t615 * t333;
    let t8835 = t511 * t8834;
    let t8842 = t515 * t615 * t352;
    let t8849 = t71 * t1685;
    let t8850 = t8849 * t131;
    let t8854 = t2338 * t356;
    let t8858 = t2164 * t574;
    let t8862 = t640 * t1656;
    (t8830, t8835, t8842, t8849, t8850, t8854, t8858, t8862)
}
