//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 414/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk414<F: Float>(t2319: F, t89: F, t1266: F, t671: F, t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t666: F, t614: F, t94: F, t659: F, t2248: F, t95: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2320 = t89 * t2319;
    let t2323 = t1266 * t671;
    let t2327 = 11.0 / 9.0 * t2281 * t107;
    let t2328 = t626 * t667;
    let t2331 = 1.0 / t655 / t106;
    let t2332 = t666 * t666;
    let t2333 = t2331 * t2332;
    let t2336 = tau0 * t614;
    let t2341 = 1.0 / t94;
    let t2342 = t659 * t659;
    let t2343 = t2341 * t2342;
    let t2346 = t95 * t2248;
    (t2320, t2323, t2327, t2328, t2331, t2332, t2333, t2336, t2341, t2342, t2343, t2346)
}
