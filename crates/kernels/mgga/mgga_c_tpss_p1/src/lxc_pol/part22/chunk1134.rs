//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1134/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1134<F: Float>(t1139: F, t12636: F, t1136: F, t1149: F, t12555: F, t12557: F, t12569: F, t12573: F, t12577: F, t1587: F, t3113: F, t3120: F, t3145: F, t4296: F, t4300: F, t4323: F, t473: F, t9730: F) -> F {
    let t12637 = t1139 * t12636;
    let t12639 = -F::cast_from(6.0_f64) * t1136 * t12569 + F::cast_from(4.0_f64) * t1136 * t12573 + F::cast_from(2.0_f64) * t1136 * t12577 - t1136 * t12637 - F::cast_from(2.0_f64) * t1149 * t12557 + t12555 * t473 - t1587 * t9730 + F::cast_from(4.0_f64) * t3113 * t4300 - F::cast_from(2.0_f64) * t3113 * t4323 + F::cast_from(2.0_f64) * t3120 * t4296 - t3145 * t4296;
    t12639
}
