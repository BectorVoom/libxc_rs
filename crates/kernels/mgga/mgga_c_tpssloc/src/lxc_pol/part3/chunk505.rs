//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 505/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk505<F: Float>(t109: F, t102: F, t662: F, t2248: F, t103: F, t100: F, t2336: F, t2343: F, t2346: F, t657: F, t660: F, t92: F, t96: F, t656: F, t2327: F, t2328: F, t2333: F, t64: F) -> (F, F, F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t2349 = 1.0 / t102;
    let t2350 = t662 * t662;
    let t2351 = t2349 * t2350;
    let t2354 = -t2248;
    let t2355 = t103 * t2354;
    let t2358 = 40.0 / 9.0 * t2336 * t96 - 50.0 / 9.0 * t657 * t660 + 10.0 / 9.0 * t92 * t2343 + 5.0 / 3.0 * t92 * t2346 + 10.0 / 9.0 * t100 * t2351 + 5.0 / 3.0 * t100 * t2355;
    let t2359 = t656 * t2358;
    let t2363 = piecewise3(t110, 0.0, t2327 + 2.0 / 3.0 * t2328 + t64 * t2333 / 4.0 - t64 * t2359 / 8.0);
    (t2349, t2350, t2351, t2354, t2355, t2358, t2359, t2363)
}
