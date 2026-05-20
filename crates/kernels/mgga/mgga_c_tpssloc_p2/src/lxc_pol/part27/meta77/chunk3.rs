//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 518/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk518<F: Float>(t1464: F, t1473: F, t1476: F, t1484: F, t1530: F, t193: F, t202: F, t680: F, t705: F, t752: F, t760: F, t765: F, t766: F, t870: F) -> F {
    let t1534 = t1530 * t193 * t202 * t870 + F::new(3.0) * t1484 * t193 * t766 + t1464 + t1473 + t1476 + t680 + t705 + t752 - t760 - t765;
    t1534
}
