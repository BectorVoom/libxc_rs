//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2104/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104<F: Float>(t12521: F, t7467: F, t81440: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F, t2332: F, t81446: F, t666: F) -> (F, F, F, F, F, F, F) {
    let t86582 = F::new(0.135e2) * t12521 * t7467;
    let t86583 = F::new(22.0) / F::new(9.0) * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86589 = F::new(4.0) / F::new(3.0) * t86588;
    let t86590 = t22470 * t4067;
    let t86591 = F::new(2.0) / F::new(3.0) * t86590;
    let t86592 = t1453 * t2332;
    let t86593 = t81446 * t86592;
    let t86595 = t4067 * t666;
    (t86582, t86583, t86586, t86589, t86591, t86593, t86595)
}
