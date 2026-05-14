//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 919/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk919<F: Float>(t5: F, t13450: F, t117: F, t4637: F, t623: F, t5314: F, t645: F, t1163: F, t4674: F, t1600: F, t3537: F, t1338: F, t4341: F, t4646: F, t600: F, t4645: F, t7594: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t13451 = piecewise3(t8, 0.0, t13450);
    let t13452 = t13451 * t117;
    let t13458 = t623 * t4637;
    let t13463 = t5314 * t645;
    let t13470 = t1163 * t4674;
    let t13473 = t1600 * t3537;
    let t13478 = t4341 * t1338;
    let t13483 = t600 * t4646;
    let t13485 = t7594 * t4645;
    (t13451, t13452, t13458, t13463, t13470, t13473, t13478, t13483, t13485)
}
