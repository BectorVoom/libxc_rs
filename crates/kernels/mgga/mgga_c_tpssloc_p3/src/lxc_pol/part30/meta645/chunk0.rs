//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2057/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057<F: Float>(t1611: F, t23528: F, t23436: F, t4640: F, t14507: F, t23536: F, t23540: F, t23433: F, t4630: F, t10189: F, t1920: F, t4343: F) -> (F, F, F, F, F, F) {
    let t88584 = t1611 * t23528;
    let t88591 = t4640 * t23436;
    let t88594 = t14507 * t23536;
    let t88600 = t14507 * t23540;
    let t88604 = t23433 * t4630 / F::new(1152.0);
    let t88622 = t1920 * t10189 * t4343 / F::new(216.0);
    (t88584, t88591, t88594, t88600, t88604, t88622)
}
