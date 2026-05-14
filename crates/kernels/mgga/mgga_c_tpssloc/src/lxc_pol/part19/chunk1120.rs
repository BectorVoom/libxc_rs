//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1120/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120<F: Float>(t40: F, t52: F, t2244: F, t2250: F, t2291: F, t39097: F, t39103: F, t39110: F, t634: F, t75: F, t767: F, t9258: F, t9499: F, t2298: F, t638: F, t771: F, t78: F, t9508: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t40833 = piecewise3(t146, 0.0, -56.0 / 81.0 * t2291 * t39097 + 16.0 / 9.0 * t634 * t2244 * t2250 - 2.0 / 3.0 * t75 * t39103 - 8.0 / 9.0 * t9499 * t9258 + 2.0 / 3.0 * t767 * t39110);
    let t40846 = piecewise3(t150, 0.0, -56.0 / 81.0 * t2298 * t39097 - 16.0 / 9.0 * t638 * t2244 * t2250 - 2.0 / 3.0 * t78 * t39103 - 8.0 / 9.0 * t9508 * t9258 - 2.0 / 3.0 * t771 * t39110);
    (t40833, t40846)
}
