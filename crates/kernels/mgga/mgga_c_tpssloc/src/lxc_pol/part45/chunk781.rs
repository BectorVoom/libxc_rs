//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 781/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk781<F: Float>(t6712: F, t995: F, t1941: F, t3077: F, t1942: F, t3082: F, t344: F, t40: F, t1009: F, t6740: F, t1015: F, t6746: F) -> (F, F, F, F, F) {
    let t23463 = t6712 * t995;
    let t23465 = t3077 * t1941;
    let t23469 = t1942 * t3082 / F::new(6912.0);
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    let t23472 = t6740 * t23471;
    let t23473 = t1015 * t6746;
    (t23463, t23465, t23469, t23472, t23473)
}
