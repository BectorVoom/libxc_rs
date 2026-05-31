//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 776/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk776<F: Float>(t112: F, t9346: F, t111: F, t2311: F, t2319: F, t649: F, t107: F, t2585: F, t2281: F, t667: F, t2333: F, t626: F) -> (F, F, F, F, F, F) {
    let t9347 = t9346 * t112;
    let t9348 = t2311 * t111;
    let t9351 = t649 * t2319;
    let t9358 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t2585 * t107;
    let t9359 = t2281 * t667;
    let t9361 = t626 * t2333;
    (t9347, t9348, t9351, t9358, t9359, t9361)
}
