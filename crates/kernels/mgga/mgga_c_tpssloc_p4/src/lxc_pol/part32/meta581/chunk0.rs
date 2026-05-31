//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1963/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963<F: Float>(t2123: F, t6146: F, t2144: F, t6150: F, t1720: F, t8054: F, t5971: F, t7286: F, t24595: F, t27426: F, t8002: F, t2121: F, t2124: F, t27755: F, t27770: F, t29671: F, t29674: F, t29678: F, t498: F, t7283: F, t7999: F, t8011: F) -> (F, F, F, F, F, F, F) {
    let t29682 = t6146 * t2123;
    let t29685 = t6150 * t2144;
    let t29687 = t1720 * t8054;
    let t29690 = t7286 * t5971;
    let t29691 = t24595 * t29690;
    let t29694 = t27426 * t8002;
    let t29699 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t29671 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t29674 - F::cast_from(0.54831135561607547884e-2_f64) * t27755 + F::cast_from(0.80418998823691070228e-1_f64) * t29678 * t2124 - F::cast_from(0.54831135561607547884e-2_f64) * t27770 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t29682 + t29685 * t498 + F::cast_from(2.0_f64) * t29687 * t498 + F::cast_from(0.36554090374405031923e-2_f64) * t7283 * t29691 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t29694 - F::cast_from(0.43864908449286038306e-1_f64) * t7999 * t8011;
    (t29682, t29685, t29687, t29690, t29691, t29694, t29699)
}
