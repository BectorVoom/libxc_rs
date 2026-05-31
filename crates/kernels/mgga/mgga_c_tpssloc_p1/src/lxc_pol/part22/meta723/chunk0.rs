//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2368/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2368<F: Float>(t21160: F, t699: F, t21167: F, t47705: F, t47707: F, t48103: F, t49139: F, t49144: F, t68442: F, t68444: F, t68446: F, t68448: F) -> (F, F, F) {
    let t68452 = t699 * t21160;
    let t68454 = t699 * t21167;
    let t68457 = F::cast_from(0.60385e0_f64) * t68442 + F::cast_from(0.10064166666666666667e0_f64) * t68444 + F::cast_from(0.11182407407407407407e0_f64) * t68446 - F::cast_from(0.40256666666666666667e0_f64) * t68448 + F::cast_from(0.80513333333333333336e0_f64) * t47705 - F::cast_from(0.26837777777777777779e0_f64) * t47707 - t49139 - t49144 - F::cast_from(0.33114e0_f64) * t68452 + F::cast_from(0.5519e-1_f64) * t68454 + F::cast_from(0.73586666666666666667e0_f64) * t48103;
    (t68452, t68454, t68457)
}
