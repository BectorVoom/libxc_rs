//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 759/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk759<F: Float>(t52: F, t197: F, t2440: F, t607: F, t2250: F, t76: F, t9258: F, t9288: F, t9436: F, t145: F, t185: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t150 = t52 <= zeta_threshold;
    let t9438 = 1.0 / t197 / t52;
    let t9441 = t2440 * t607;
    let t9447 = piecewise3(t150, 0.0, 8.0 / 27.0 * t9438 * t9288 + 4.0 / 3.0 * t9441 * t2250 - 4.0 / 3.0 * t76 * t9258);
    let t9448 = t9436 + t9447;
    let t9449 = t145 * t9448;
    let t9450 = t9449 * t185;
    let t9452 = 1.0 / t2409 / t138;
    let t9453 = t125 * t9452;
    let t9454 = t2412 * t701;
    (t9438, t9441, t9448, t9449, t9450, t9452, t9453, t9454)
}
