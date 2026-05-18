//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 993/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk993<F: Float>(t248: F, t3516: F, t3570: F, t3515: F, t11154: F, t3585: F, t3493: F, t486: F, t4978: F, t4582: F, t3576: F, t3604: F) -> (F, F, F, F, F, F) {
    let t11651 = t248 * t3570 * t3516;
    let t11652 = t3515 * t11651;
    let t11655 = t248 * t3585 * t11154;
    let t11660 = t486 * t3493;
    let t11661 = t11660 * t4978;
    let t11662 = t4582 * t11661;
    let t11665 = t3604 * t3576;
    (t11651, t11652, t11655, t11660, t11662, t11665)
}
