//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 907/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk907<F: Float>(t5493: F, t652: F, t8595: F, t33620: F, t4028: F, t22574: F, t33357: F, t33899: F, t1983: F, t33136: F, t7940: F, t28817: F, t8607: F, t28823: F, t127162: F, t26161: F, t26558: F) -> (F, F, F, F, F, F, F) {
    let t128452 = 2.0 * t652 * t8595 * t5493;
    let t128454 = 4.0 * t4028 * t33620;
    let t128457 = 6.0 * t22574 * t33899 * t33357;
    let t128460 = 2.0 * t1983 * t7940 * t33136;
    let t128464 = 6.0 * t8607 * t28817;
    let t128466 = 2.0 * t8607 * t28823;
    let t128474 = 4.0 * t26161 * t26558 * t127162;
    (t128452, t128454, t128457, t128460, t128464, t128466, t128474)
}
