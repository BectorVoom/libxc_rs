//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1365/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1365<F: Float>(t10482: F, t3032: F, t131: F, t2261: F, t350: F, t6794: F, t23598: F, t614: F, t23610: F, t23665: F, t3127: F, t82514: F) -> (F, F, F, F, F) {
    let t82516 = t3032 * t10482;
    let t82527 = t2261 * t6794 * t131 * t350;
    let t82534 = t614 * t23598 * t131 * t350;
    let t82539 = t23665 * t23610;
    let t82541 = t82514 * t3127;
    (t82516, t82527, t82534, t82539, t82541)
}
