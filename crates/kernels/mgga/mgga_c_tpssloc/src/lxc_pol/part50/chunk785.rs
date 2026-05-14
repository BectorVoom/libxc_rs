//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 785/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk785<F: Float>(t225: F, t4555: F, t3701: F, t5356: F, t12461: F, t1845: F, t5213: F, t5211: F, t1372: F, t1824: F, t5286: F, t562: F, t1351: F, t1799: F, t120: F, t3792: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14555 = t4555 * t225;
    let t15868 = t5356 * t3701;
    let t15899 = t1845 * t12461;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16036 = t1372 * t1824;
    let t16040 = t562 * t5286;
    let t16225 = t1799 * t1351;
    let t16242 = t120 * t5286;
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    (t14555, t15868, t15899, t16022, t16030, t16036, t16040, t16225, t16242, t16306, t16311)
}
