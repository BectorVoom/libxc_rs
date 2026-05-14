//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1214/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1214<F: Float>(t10904: F, t11002: F, t10508: F, t248: F, t3130: F, t3132: F, t10969: F, t121: F, t10305: F, t1041: F, t1015: F, t3033: F, t42520: F, t3142: F, t698: F, t973: F) -> (F, F, F, F, F) {
    let t42582 = t10904 * t11002;
    let t42586 = t3130 * t248 * t10508 * t3132;
    let t42592 = t121 * t10969;
    let t42595 = t1041 * t248 * t42592 * t10305;
    let t42600 = t3033 * t1015 * t42520;
    let t42610 = t973 * t698 * t3142;
    (t42582, t42586, t42595, t42600, t42610)
}
