//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1281/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1281<F: Float>(t26103: F, t7461: F, t25980: F, t6517: F, t26179: F, t8327: F, t31058: F, t7458: F, t652: F, t6534: F, t7670: F, t19456: F) -> (F, F, F, F, F, F) {
    let t120714 = t26103 * t7461;
    let t120716 = t6517 * t25980;
    let t120719 = F::new(2.0) * t26179 * t8327;
    let t120721 = F::new(2.0) * t7458 * t31058;
    let t120723 = t652 * t7670 * t6534;
    let t120728 = F::new(2.0) * t19456 * t8327;
    (t120714, t120716, t120719, t120721, t120723, t120728)
}
