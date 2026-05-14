//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1091/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1091<F: Float>(t39518: F, t39521: F, t39523: F, t39529: F, t39531: F, t39533: F, t39539: F, t39541: F, t39549: F, t39563: F, t39570: F, t39572: F, t39582: F, t39585: F, t39590: F, t39593: F, t39595: F, t39597: F, t39602: F, t39604: F, t39606: F, t39608: F, t39610: F, t39612: F, t39615: F) -> (F, F) {
    let t40217 = t39518 - t39521 - t39523 - t39529 - t39531 - t39533 + t39539 - t39541 + t39549 + t39563 + t39570 - t39572;
    let t40218 = t39582 - t39585 + t39590 - t39593 + t39595 - t39597 + t39602 + t39604 - t39606 - t39608 + t39610 - t39612 + t39615;
    (t40217, t40218)
}
