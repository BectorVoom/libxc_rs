//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2571/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2571<F: Float>(t14744: F, t15402: F, t3447: F, t1174: F, t135: F, t15359: F, t11589: F, t15293: F, t15382: F, t44525: F, t11588: F, t4928: F) -> (F, F, F, F, F) {
    let t51995 = t3447 * t15402 * t14744;
    let t52013 = t1174 * t135 * t15359;
    let t52019 = t3447 * t11589 * t15293;
    let t52022 = t3447 * t44525 * t15382;
    let t52036 = t11588 * t4928;
    (t51995, t52013, t52019, t52022, t52036)
}
