//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1908/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1908<F: Float>(t22960: F, t98007: F, t5660: F, t776: F, t67164: F, t1408: F, t4119: F, t1530: F, t4303: F, t25373: F, t67123: F, t5544: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98008 = t22960 * t98007;
    let t98011 = t5660 * t776;
    let t98012 = t22960 * t98011;
    let t98015 = t22960 * t67164;
    let t98020 = t1408 * t4119;
    let t98030 = t1530 * t4303;
    let t98031 = t25373 * t98030;
    let t98034 = t22960 * t67123;
    let t98046 = t606 * t5544;
    (t98008, t98011, t98012, t98015, t98020, t98030, t98031, t98034, t98046)
}
