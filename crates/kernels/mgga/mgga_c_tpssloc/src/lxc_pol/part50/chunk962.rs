//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 962/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk962<F: Float>(t31047: F, t6879: F, t1983: F, t1976: F, t6534: F, t652: F, t2314: F, t8327: F, t4034: F, t1266: F, t8326: F, t113: F, t1869: F, t30989: F, t30993: F, t30995: F, t31029: F, t31034: F, t31038: F, t31039: F, t31041: F, t31046: F, t510: F, t650: F, t6515: F, t6862: F, t8313: F, t8329: F, t8439: F) -> (F, F, F, F, F, F, F) {
    let t31048 = t31047 * t6879;
    let t31050 = 3.0 * t1983 * t31048;
    let t31051 = t1976 * t6534;
    let t31052 = t652 * t31051;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0 * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = 2.0 * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = 2.0 * t31059;
    let t31061 = -t113 * t30989 - t1266 * t8313 - 2.0 * t1869 * t6862 - 2.0 * t1976 * t6515 - t31029 * t510 - t650 * t8439 - t30993 - t30995 - t31034 - t31038 + 6.0 * t31039 - 2.0 * t31041 + t31046 + t31050 - 4.0 * t31052 - t31055 - t31057 - t31060 - t8329;
    (t31048, t31051, t31055, t31057, t31058, t31060, t31061)
}
