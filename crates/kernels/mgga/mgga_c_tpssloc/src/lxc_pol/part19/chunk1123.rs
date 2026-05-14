//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1123/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1123<F: Float>(t10108: F, t257: F, t68: F, t2719: F, t2627: F, t2710: F, t10016: F, t252: F, t9957: F, t852: F, t9971: F, t2631: F, t2632: F, t233: F, t9970: F, t9975: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40889 = 1.0 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40891 = t2719 * t2719;
    let t40895 = t2627 * t2710;
    let t40904 = t10016 * t68;
    let t40909 = t252 * t9957;
    let t40917 = t9971 * t852;
    let t40925 = t2631 * t2631;
    let t40926 = t40925 * t2632;
    let t40931 = 1.0 / t9970 / t233;
    let t40932 = t40931 * t252;
    let t40933 = t2632 * t2632;
    let t40934 = t40925 * t40933;
    let t40938 = t40925 * t9975;
    (t40890, t40891, t40895, t40904, t40909, t40917, t40925, t40926, t40931, t40932, t40934, t40938)
}
