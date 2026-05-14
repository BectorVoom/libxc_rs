//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1060/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1060<F: Float>(t2221: F, t3824: F, t12132: F, t592: F, t68: F, t6924: F, t1336: F, t1339: F, t2691: F, t10021: F, t154: F, t59: F, t3749: F, t598: F, t535: F, t795: F) -> (F, F, F, F, F, F, F, F) {
    let t40227 = t2221 * t3824;
    let t40228 = 72.0 * t40227;
    let t40230 = 16.0 * t592 * t12132;
    let t40253 = t68 * t6924;
    let t40281 = t1336 * t1339 * t2691;
    let t40341 = t59 * t10021 * t154;
    let t40343 = 0.99537037037037037035e-1 * t40341 * t3749;
    let t40344 = t59 * t598;
    let t40347 = 0.11265432098765432099e0 * t40344 * t535 * t795;
    (t40228, t40230, t40253, t40281, t40341, t40343, t40344, t40347)
}
