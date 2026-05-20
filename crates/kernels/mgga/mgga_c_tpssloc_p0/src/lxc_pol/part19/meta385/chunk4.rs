//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1446/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1446<F: Float>(t11517: F, t11539: F, t1174: F, t11521: F, t3431: F, t1184: F, t15394: F, t11147: F, t460: F, t9288: F, t11588: F, t3469: F) -> (F, F, F, F, F) {
    let t44499 = t1174 * t11539 * t11517;
    let t44502 = t1174 * t3431 * t11521;
    let t44504 = t15394 * t1184;
    let t44505 = t460 * t11147;
    let t44506 = t44505 * t9288;
    let t44510 = t11588 * t3469;
    (t44499, t44502, t44504, t44506, t44510)
}
