//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2128/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2128<F: Float>(t3147: F, t698: F, t973: F, t10981: F, t2960: F, t10984: F, t1004: F, t10956: F, t10863: F, t3053: F, t10516: F, t3113: F) -> (F, F, F, F, F, F) {
    let t42613 = t973 * t698 * t3147;
    let t42619 = t2960 * t10981;
    let t42622 = t2960 * t10984;
    let t42648 = t1004 * t10956;
    let t42651 = t10863 * t3053;
    let t42653 = t3113 * t10516;
    (t42613, t42619, t42622, t42648, t42651, t42653)
}
