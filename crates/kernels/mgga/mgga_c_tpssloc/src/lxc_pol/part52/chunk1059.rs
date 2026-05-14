//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1059/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1059<F: Float>(t7000: F, t8450: F, t12461: F, t1388: F, t8493: F, t1983: F, t532: F, t8488: F, t6879: F, t1976: F, t6534: F, t652: F, t2314: F, t8327: F, t4034: F, t1266: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31041 = t8450 * t7000;
    let t31043 = t12461 * t1388;
    let t31044 = t8493 * t31043;
    let t31046 = 2.0 * t1983 * t31044;
    let t31047 = t532 * t8488;
    let t31048 = t31047 * t6879;
    let t31050 = 3.0 * t1983 * t31048;
    let t31051 = t1976 * t6534;
    let t31052 = t652 * t31051;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0 * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = 2.0 * t31056;
    let t31058 = t1266 * t8326;
    (t31041, t31044, t31046, t31047, t31048, t31050, t31051, t31052, t31055, t31057, t31058)
}
