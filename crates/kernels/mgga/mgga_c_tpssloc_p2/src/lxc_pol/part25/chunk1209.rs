//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1209/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1209<F: Float>(t2047: F, t9971: F, t2627: F, t7084: F, t24251: F, t24270: F, t24273: F, t2617: F, t2633: F, t4182: F, t4281: F, t7101: F, t7102: F, t812: F, t81617: F, t81623: F, t81627: F, t81630: F, t81633: F, t81637: F, t81642: F, t81645: F, t81648: F, t81653: F, t84842: F, t9612: F, t9661: F, t9976: F) -> F {
    let t84953 = t9971 * t2047;
    let t84962 = t2627 * t7084;
    let t84981 = -F::cast_from(0.11514538467937585055e0_f64) * t81617 - F::cast_from(3.0_f64) * t9612 * t7102 - F::cast_from(6.0_f64) * t812 * t84953 * t9976 - F::cast_from(3.0_f64) * t2617 * t24273 + F::cast_from(6.0_f64) * t4281 * t84842 * t4182 + F::cast_from(6.0_f64) * t812 * t84962 * t2633 + F::cast_from(0.46058153871750340221e0_f64) * t81623 - F::cast_from(0.3289868133696452873e-1_f64) * t81627 + F::cast_from(0.49348022005446793095e-1_f64) * t81630 - F::cast_from(0.76763589786250567036e0_f64) * t81633 - F::cast_from(0.9869604401089358619e-1_f64) * t81637 - F::cast_from(0.14804406601634037928e0_f64) * t81642 - F::cast_from(6.0_f64) * t2617 * t24270 - t812 * t7101 * t9661 + F::cast_from(0.9869604401089358619e-1_f64) * t81645 - F::cast_from(0.49348022005446793095e-1_f64) * t81648 - F::cast_from(0.9869604401089358619e-1_f64) * t81653 - F::cast_from(3.0_f64) * t2617 * t24251;
    t84981
}
