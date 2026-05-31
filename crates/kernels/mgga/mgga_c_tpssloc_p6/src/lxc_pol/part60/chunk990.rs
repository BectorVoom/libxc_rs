//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 990/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk990<F: Float>(t3941: F, t5493: F, t8326: F, t1458: F, t1851: F, t576: F, t33191: F, t8657: F, t33185: F, t33656: F, t33659: F, t24465: F, t28896: F) -> (F, F, F, F, F, F, F) {
    let t127627 = F::cast_from(27.0_f64) * t3941 * t8326 * t5493;
    let t127630 = t1851 * t1458;
    let t127643 = t576 * t5493;
    let t127646 = F::cast_from(27.0_f64) * t33191;
    let t127669 = F::cast_from(27.0_f64) * t127643 * t8657;
    let t127671 = F::cast_from(54.0_f64) * t33185 * t33656;
    let t127673 = F::cast_from(54.0_f64) * t33185 * t33659;
    let t127677 = F::cast_from(54.0_f64) * t24465 * t28896;
    (t127627, t127630, t127646, t127669, t127671, t127673, t127677)
}
