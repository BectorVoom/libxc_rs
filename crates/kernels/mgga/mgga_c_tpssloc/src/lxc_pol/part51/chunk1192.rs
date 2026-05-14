//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1192/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1192<F: Float>(t22827: F, t26297: F, t6943: F, t26301: F, t26322: F, t6936: F, t1831: F, t31176: F, t1369: F, t32717: F, t31165: F, t5314: F, t8466: F, t22804: F, t32711: F, t22759: F, t26318: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120366 = t22827 * t6943 * t26297;
    let t120369 = t22827 * t6943 * t26301;
    let t120372 = t6936 * t6943 * t26322;
    let t120375 = t31176 * t1831;
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    let t120383 = t22804 * t32711;
    let t120388 = t6936 * t22759 * t26318;
    (t120366, t120369, t120372, t120375, t120377, t120379, t120381, t120383, t120388)
}
