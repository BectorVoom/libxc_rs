//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1802/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1802<F: Float>(t23072: F, t23083: F, t23069: F, t2610: F, t2690: F, t6612: F, t812: F, t831: F, t23041: F, t2686: F, t59: F, t9971: F) -> (F, F, F, F, F, F) {
    let t81797 = t23083 * t23072;
    let t81799 = t23069 * t2610;
    let t81807 = t812 * t6612 * t2690;
    let t81808 = t81807 * t831;
    let t81810 = t23041 * t2686;
    let t81816 = t9971 * t59;
    (t81797, t81799, t81807, t81808, t81810, t81816)
}
