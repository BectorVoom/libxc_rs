//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1045/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1045<F: Float>(t24574: F, t32462: F, t32459: F, t477: F, t7348: F, t32551: F, t3640: F, t11947: F, t8900: F, t43706: F, t8904: F, t2174: F, t7415: F, t2169: F, t7426: F, t1395: F, t8927: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118162 = t24574 * t32462;
    let t118173 = t24574 * t32459;
    let t118175 = t477 * t7348;
    let t118229 = t32551 * t3640;
    let t118233 = t8900 * t11947;
    let t118251 = t8904 * t43706;
    let t118335 = t7415 * t2174;
    let t118337 = t2169 * t7426;
    let t118345 = t1395 * t8927;
    (t118162, t118173, t118175, t118229, t118233, t118251, t118335, t118337, t118345)
}
