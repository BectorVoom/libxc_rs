//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 548/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk548<F: Float>(t15397: F, t2067: F, t3369: F, t14230: F, t209: F, t605: F, t664: F, t515: F, t1971: F, t1970: F, t26: F, t14163: F, t15037: F, t15041: F, t15044: F, t15047: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15398 = t2067 * t15397;
    let t15399 = t3369 * t15398;
    let t15400 = t14230 * t15399;
    let t15403 = t664 * t605 * t209;
    let t15404 = t515 * t15403;
    let t15405 = t1971 * t15404;
    let t15406 = t1970 * t15405;
    let t15409 = t26 * t605 * t209;
    let t15410 = t2067 * t15409;
    let t15411 = t3369 * t15410;
    let t15412 = t14163 * t15411;
    let t15426 = 0.30487649791575028312e-3 * t15037;
    let t15427 = 0.30487649791575028312e-3 * t15041;
    let t15428 = 0.16263363996404810741e-4 * t15044;
    let t15429 = 0.16263363996404810741e-4 * t15047;
    (t15399, t15400, t15405, t15406, t15411, t15412, t15426, t15427, t15428, t15429)
}
