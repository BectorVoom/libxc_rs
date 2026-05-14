//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1228/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1228<F: Float>(t42308: F, t974: F, t344: F, t41666: F, t10224: F, t2999: F, t973: F, t2978: F, t698: F, t2981: F, t10263: F, t2971: F, t2402: F, t976: F, t979: F, t2955: F, t2967: F) -> (F, F, F, F, F, F, F) {
    let t42861 = t974 * t42308;
    let t42862 = t344 * t41666;
    let t42873 = t973 * t10224 * t2999;
    let t42875 = t698 * t2978;
    let t42877 = t973 * t42875 * t2981;
    let t42889 = t10263 * t2971;
    let t42891 = t2402 * t976;
    let t42893 = t973 * t42891 * t979;
    let t42895 = t2955 * t2967;
    (t42861, t42862, t42873, t42877, t42889, t42893, t42895)
}
