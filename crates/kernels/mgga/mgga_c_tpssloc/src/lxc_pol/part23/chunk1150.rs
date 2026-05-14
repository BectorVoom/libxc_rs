//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1150/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1150<F: Float>(t13969: F, t21486: F, t3130: F, t10422: F, t21565: F, t3070: F, t21126: F, t2970: F, t973: F, t21569: F, t42488: F, t10231: F, t21122: F, t21689: F, t225: F, t21669: F) -> (F, F, F, F, F, F, F) {
    let t70805 = t3130 * t13969 * t21486;
    let t70846 = t3070 * t10422 * t21565;
    let t70867 = t973 * t2970 * t21126;
    let t70912 = t3070 * t42488 * t21569;
    let t70929 = t973 * t10231 * t21122;
    let t70978 = t21689 * t225;
    let t70980 = t21669 * t225;
    (t70805, t70846, t70867, t70912, t70929, t70978, t70980)
}
