//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1309/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1309<F: Float>(t19956: F, t2814: F, t6185: F, t9133: F, t11792: F, t1485: F, t18196: F, t18200: F, t198: F, t19960: F, t19965: F, t2807: F, t2811: F, t330: F, t4019: F, t4023: F, t45417: F, t5652: F, t61584: F, t61588: F, t61595: F, t64543: F, t64617: F, t64670: F, t64725: F, t993: F, t995: F) -> (F,) {
    let t64731 = t19956 * t2814;
    let t64735 = t6185 * t9133;
    let t64762 = t198 * t330 * (t64543 + t64617 + t64670 + t64725) * t995 - 2.0 * t4023 * t64731 * t993 + 2.0 * t4023 * t64735 * t2811 - t4023 * t19960 * t2807 - t4023 * t61584 * t1485 + 4.0 * t4023 * t61588 * t19965 - 2.0 * t4023 * t18196 * t4019 - 6.0 * t4023 * t61595 * t1485 * t2811 + 4.0 * t4023 * t18200 * t45417 + 2.0 * t4023 * t18200 * t1485 * t2807 - t4023 * t5652 * t11792;
    (t64762,)
}
