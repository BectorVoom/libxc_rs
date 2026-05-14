//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1277/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1277<F: Float>(t25994: F, t7042: F, t31537: F, t7802: F, t31540: F, t27226: F, t8526: F, t1983: F, t33335: F, t6999: F, t8606: F, t8944: F, t26164: F, t33211: F, t7057: F, t649: F, t7467: F) -> (F, F, F, F, F, F, F, F) {
    let t122610 = t7042 * t25994;
    let t122623 = 2.0 * t31537 * t7802;
    let t122625 = 2.0 * t31540 * t7802;
    let t122627 = 2.0 * t8526 * t27226;
    let t122645 = t1983 * t33335 * t6999;
    let t122654 = t8606 * t8944;
    let t122656 = 2.0 * t122654 * t26164;
    let t122659 = 2.0 * t33211 * t7057;
    let t122660 = t649 * t7467;
    (t122610, t122623, t122625, t122627, t122645, t122656, t122659, t122660)
}
