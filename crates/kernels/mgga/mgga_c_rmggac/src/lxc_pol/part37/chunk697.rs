//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 697/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk697<F: Float>(t1476: F, t236: F, t14117: F, t68906: F, t69839: F, t9146: F, t14124: F, t201: F, t457: F, t618: F, t68422: F, t14131: F, t9170: F, t21714: F, t551: F, t14125: F, t515: F, t570: F) -> (F, F, F, F, F, F, F) {
    let t74376 = t236 * t1476;
    let t74378 = t68906 * t14117 * t74376;
    let t74381 = t69839 * t14117 * t9146;
    let t74387 = t14124 * t68422 * t236 * t618 * t457 * t201;
    let t74390 = t14131 * t68422 * t9170;
    let t74396 = t14124 * t21714 * t236 * t551 * t457 * t201;
    let t74403 = t14124 * t14125 * t515 * t570 * t457 * t201;
    (t74376, t74378, t74381, t74387, t74390, t74396, t74403)
}
