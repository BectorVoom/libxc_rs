//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 973/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk973<F: Float>(t31390: F, t6547: F, t23030: F, t31381: F, t2047: F, t2631: F, t1888: F, t22996: F, t2632: F, t23110: F, t23185: F, t31385: F) -> (F, F, F, F, F) {
    let t114670 = t6547 * t31390;
    let t114672 = t23030 * t31381;
    let t114673 = F::new(0.26044789391763585244e-1) * t114672;
    let t114674 = t2047 * t2631;
    let t114677 = t1888 * t22996 * t114674 * t2632;
    let t114680 = t23185 * t23110 * t31385;
    (t114670, t114673, t114674, t114677, t114680)
}
