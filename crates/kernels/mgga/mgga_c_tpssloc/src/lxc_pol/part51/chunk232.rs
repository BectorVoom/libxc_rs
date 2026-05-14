//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 232/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk232<F: Float>(t193: F, t202: F, t680: F, t705: F, t710: F, t719: F, t752: F, t755: F, t760: F, t765: F, t766: F, t776: F, t868: F, t870: F) -> (F,) {
    let t873 = t193 * t202 * t868 * t870 + 3.0 * t193 * t766 * t776 + t680 + t705 + t710 + t719 + t752 + t755 - t760 - t765;
    (t873,)
}
