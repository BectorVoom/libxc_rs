//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 414/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk414<F: Float>(t8923: F, t8955: F, t8991: F, t9028: F, t495: F, t570: F, t515: F, t498: F, t5144: F, t132: F, t577: F, t1392: F, t202: F, t461: F, t674: F, t2185: F, t2411: F) -> (F, F, F, F, F, F, F, F) {
    let t9030 = t8923 + t8955 + t8991 + t9028;
    let t9044 = t570 * t495;
    let t9045 = t515 * t9044;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9054 = t515 * t5144;
    let t9081 = t577 * t132;
    let t9085 = t1392 * t202;
    let t9086 = t9085 * t461;
    let t9087 = t9086 * t674;
    let t9090 = t2411 * t2185;
    (t9030, t9045, t9050, t9054, t9081, t9086, t9087, t9090)
}
