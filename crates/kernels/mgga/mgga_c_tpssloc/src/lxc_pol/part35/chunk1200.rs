//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1200/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1200<F: Float>(t131: F, t467: F, t5415: F, t6794: F, t29754: F, t85853: F, t29624: F, t7327: F, t24667: F, t6252: F, t1653: F, t8039: F, t85822: F, t24574: F, t29741: F, t29614: F) -> (F, F, F, F, F, F, F) {
    let t103581 = t5415 * t6794 * t131 * t467;
    let t103610 = t85853 * t29754;
    let t103687 = t29624 * t7327;
    let t103694 = t24667 * t6252;
    let t103699 = t85822 * t1653 * t8039;
    let t103710 = t24574 * t29741;
    let t103723 = t29614 * t7327;
    (t103581, t103610, t103687, t103694, t103699, t103710, t103723)
}
