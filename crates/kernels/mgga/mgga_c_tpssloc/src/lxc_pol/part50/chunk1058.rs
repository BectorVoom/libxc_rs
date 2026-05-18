//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1058/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1058<F: Float>(t31056: F, t1266: F, t8326: F, t652: F, t113: F, t1869: F, t1976: F, t30989: F, t30993: F, t30995: F, t31029: F, t31034: F, t31038: F, t31039: F, t31041: F, t31046: F, t31050: F, t31052: F, t31055: F, t510: F, t650: F, t6515: F, t6862: F, t8313: F, t8329: F, t8439: F) -> (F, F, F, F) {
    let t31057 = F::new(2.0) * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = F::new(2.0) * t31059;
    let t31061 = -t113 * t30989 - t1266 * t8313 - F::new(2.0) * t1869 * t6862 - F::new(2.0) * t1976 * t6515 - t31029 * t510 - t650 * t8439 - t30993 - t30995 - t31034 - t31038 + F::new(6.0) * t31039 - F::new(2.0) * t31041 + t31046 + t31050 - F::new(4.0) * t31052 - t31055 - t31057 - t31060 - t8329;
    (t31057, t31058, t31060, t31061)
}
