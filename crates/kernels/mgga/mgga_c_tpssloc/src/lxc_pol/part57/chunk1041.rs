//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1041/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1041<F: Float>(t128639: F, t128663: F, t128701: F, t128726: F, t128761: F, t128789: F, t128818: F, t128902: F, t1390: F, t1983: F, t533: F, t28821: F, t8641: F) -> (F, F) {
    let t128908 = t1983 * t533 * (t128639 + t128663 + t128701 + t128726 + t128761 + t128789 + t128818 + t128902) * t1390;
    let t128909 = t28821 * t8641;
    (t128908, t128909)
}
