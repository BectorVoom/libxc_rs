//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 747/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk747<F: Float>(t1530: F, t870: F, t193: F, t200: F, t1484: F, t262: F, t1877: F, t202: F, t2373: F, t2377: F, t2522: F, t4097: F, t4099: F, t4100: F, t4103: F, t4119: F, t4198: F, t4201: F, t4204: F, t4207: F, t4303: F, t4307: F, t766: F, t776: F, t868: F) -> (F, F, F, F) {
    let t4310 = t1530 * t870;
    let t4314 = t193 * t200;
    let t4315 = t262 * t1484;
    let t4319 = t193 * t202 * t4303 * t870 - t1877 * t4307 * t868 + 3.0 * t193 * t4119 * t766 + 3.0 * t2522 * t4310 * t776 + 6.0 * t4314 * t4315 * t776 + t2373 + t2377 + t4097 + t4099 + t4100 + t4103 + t4198 - t4201 + t4204 + t4207;
    (t4310, t4314, t4315, t4319)
}
