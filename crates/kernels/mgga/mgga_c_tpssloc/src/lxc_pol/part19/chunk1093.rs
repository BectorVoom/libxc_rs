//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1093/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093<F: Float>(t2221: F, t3826: F, t3824: F, t12132: F, t592: F, t3696: F, t2223: F, t39844: F, t39846: F, t39852: F, t39854: F, t39856: F, t39858: F, t40222: F, t40224: F, t68: F, t6924: F) -> (F, F, F, F, F, F, F) {
    let t40225 = t2221 * t3826;
    let t40226 = 144.0 * t40225;
    let t40227 = t2221 * t3824;
    let t40228 = 72.0 * t40227;
    let t40230 = 16.0 * t592 * t12132;
    let t40231 = t2221 * t3696;
    let t40232 = 72.0 * t40231;
    let t40233 = t2223 * t3696;
    let t40234 = 192.0 * t40233;
    let t40235 = t39844 + t39846 - t39852 + t39854 + t39856 - t39858 + t40222 + t40224 + t40226 + t40228 - t40230 + t40232 - t40234;
    let t40253 = t68 * t6924;
    (t40226, t40228, t40230, t40232, t40234, t40235, t40253)
}
