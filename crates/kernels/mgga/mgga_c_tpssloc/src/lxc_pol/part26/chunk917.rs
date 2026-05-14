//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 917/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk917<F: Float>(t1222: F, t3567: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t248: F, t3516: F, t3570: F, t3515: F, t11154: F, t3585: F, t3493: F, t4978: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11642 = t3567 * t1222;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / 10368.0;
    let t11651 = t248 * t3570 * t3516;
    let t11652 = t3515 * t11651;
    let t11655 = t248 * t3585 * t11154;
    let t11660 = t486 * t3493;
    let t11661 = t11660 * t4978;
    (t11642, t11644, t11647, t11649, t11651, t11652, t11655, t11660, t11661)
}
