//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 906/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk906<F: Float>(t33214: F, t7802: F, t29211: F, t8526: F, t115262: F, t1983: F, t28826: F, t120955: F, t7687: F, t33335: F, t5161: F, t33366: F, t7685: F, t5450: F, t8595: F, t2075: F, t28017: F, t652: F) -> (F, F, F, F, F, F, F, F) {
    let t128420 = 4.0 * t33214 * t7802;
    let t128422 = 2.0 * t8526 * t29211;
    let t128429 = 6.0 * t1983 * t115262 * t28826;
    let t128438 = 6.0 * t1983 * t120955 * t7687;
    let t128441 = 2.0 * t1983 * t33335 * t5161;
    let t128443 = 2.0 * t7685 * t33366;
    let t128444 = t5450 * t8595;
    let t128449 = 2.0 * t652 * t2075 * t28017;
    (t128420, t128422, t128429, t128438, t128441, t128443, t128444, t128449)
}
