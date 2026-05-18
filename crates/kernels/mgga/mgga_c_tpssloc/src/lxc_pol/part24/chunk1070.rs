//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1070/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1070<F: Float>(t12303: F, t3870: F, t820: F, t12189: F, t1329: F, t3726: F, t3770: F, t119: F, t12012: F, t210: F, t12211: F, t3766: F) -> (F, F, F, F, F) {
    let t12305 = t3870 * t820 * t12303;
    let t12308 = t12189 * t1329;
    let t12310 = t3726 * t3770;
    let t12313 = t210 * t119 * t12012;
    let t12317 = t12211 * t3766;
    (t12305, t12308, t12310, t12313, t12317)
}
