//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 857/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk857<F: Float>(t20371: F, t20679: F, t20692: F, t20696: F, t1458: F, t6287: F, t1774: F, t5493: F, t20347: F, t510: F, t16578: F, t12861: F) -> (F, F, F, F, F, F) {
    let t20698 = t20371 + t20679 + t20692 + t20696;
    let t20702 = t6287 * t1458;
    let t20717 = t1774 * t5493;
    let t20720 = t510 * t20347;
    let t20723 = F::new(3.0) * t16578;
    let t20724 = F::new(3.0) * t12861;
    (t20698, t20702, t20717, t20720, t20723, t20724)
}
