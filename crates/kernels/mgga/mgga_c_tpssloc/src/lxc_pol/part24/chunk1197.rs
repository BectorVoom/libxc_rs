//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1197/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1197<F: Float>(t23072: F, t23083: F, t23069: F, t2610: F, t23053: F, t2686: F, t23047: F, t2617: F, t2635: F, t2690: F, t6612: F, t812: F, t831: F, t23041: F, t6614: F, t9663: F) -> (F, F, F, F, F, F, F) {
    let t81797 = t23083 * t23072;
    let t81799 = t23069 * t2610;
    let t81801 = t23053 * t2686;
    let t81803 = t2617 * t23047;
    let t81804 = t81803 * t2635;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    let t81810 = t23041 * t2686;
    let t81812 = t6614 * t9663;
    (t81797, t81799, t81801, t81804, t81808, t81810, t81812)
}
