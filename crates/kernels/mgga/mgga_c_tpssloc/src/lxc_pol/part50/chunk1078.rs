//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1078/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1078<F: Float>(t23384: F, t32931: F, t61: F, t820: F, t30838: F, t354: F, t373: F, t10401: F, t113361: F, t113413: F, t113443: F, t23489: F, t25589: F, t25678: F, t3033: F, t30820: F, t30827: F, t32948: F, t32951: F, t4575: F, t4579: F, t4584: F, t4589: F, t4595: F, t4599: F, t6723: F, t6742: F, t8384: F) -> (F, F) {
    let t119238 = t23384 * t32931;
    let t119243 = t820 * t61;
    let t119248 = t354 * t30838 * t373;
    let t119274 = -t3033 * t30827 * t10401 * t119243 * t4599 / 1536.0 - t119248 * t119243 * t4584 / 1152.0 + 5.0 / 6912.0 * t119248 * t119243 * t4589 + t113413 * t4575 / 2304.0 + t113413 * t4579 / 2304.0 + t3033 * t113443 * t10401 * t119243 * t4595 / 768.0 + 0.32298204875312312685e-2 * t6723 * t32948 + 0.40372756094140390856e-3 * t25589 * t8384 + 0.40372756094140390856e-3 * t23489 * t32951 + 0.40372756094140390856e-3 * t6742 * t30820 * t25678 + t113361 / 2304.0;
    (t119238, t119274)
}
