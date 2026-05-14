//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 865/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk865<F: Float>(t33231: F, t4028: F, t28864: F, t7042: F, t33222: F, t96797: F, t28952: F, t8526: F, t29219: F, t1880: F, t29055: F, t6553: F, t6571: F, t25224: F, t33408: F, t23270: F, t25038: F, t31337: F, t5527: F) -> (F, F, F, F, F, F, F, F) {
    let t127728 = 4.0 * t4028 * t33231;
    let t127730 = 2.0 * t7042 * t28864;
    let t127736 = 4.0 * t96797 * t33222;
    let t127738 = 2.0 * t8526 * t28952;
    let t127742 = 4.0 * t8526 * t29219;
    let t127778 = t1880 * t6553 * t6571 * t29055;
    let t127786 = t1880 * t25224 * t33408;
    let t127790 = t25038 * t23270 * t31337 * t5527;
    (t127728, t127730, t127736, t127738, t127742, t127778, t127786, t127790)
}
