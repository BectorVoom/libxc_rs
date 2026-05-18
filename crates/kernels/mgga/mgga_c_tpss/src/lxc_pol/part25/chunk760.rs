//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 760/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk760<F: Float>(t1043: F, t5113: F, t1024: F, t2913: F, t5081: F, t2911: F, t2917: F, t4044: F, t5066: F, t5070: F, t5074: F, t1530: F) -> (F, F, F, F, F, F) {
    let t5114 = t5113 * t1043;
    let t5116 = F::new(1.0) * t1024 * t5114;
    let t5117 = t5081 * t2913;
    let t5119 = F::new(0.16081979498692535067e2) * t2911 * t5117;
    let t5124 = t2917 - F::new(0.11415555555555555555e-1) * t4044 - F::new(0.11415555555555555555e-1) * t5066 + F::new(0.34246666666666666666e-1) * t5070 + F::new(0.17123333333333333333e-1) * t5074;
    let t5129 = t1530 * t1530;
    (t5114, t5116, t5117, t5119, t5124, t5129)
}
