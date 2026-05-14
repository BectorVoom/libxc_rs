//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1318/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1318<F: Float>(t11502: F, t3448: F, t1184: F, t15418: F, t11571: F, t3447: F, t3469: F, t4899: F, t11570: F, t9288: F, t3450: F, t9258: F, t11584: F, t11593: F, t1174: F, t24705: F, t3449: F, t3451: F, t43719: F, t43723: F, t44499: F, t44502: F, t44504: F, t44506: F, t44512: F, t44517: F, t4908: F, t4934: F) -> (F,) {
    let t44521 = t3448 * t11502;
    let t44525 = t15418 * t1184;
    let t44527 = t3447 * t44525 * t11571;
    let t44529 = t4899 * t3469;
    let t44536 = t11570 * t9288;
    let t44540 = t3450 * t9258;
    let t44547 = -0.49999999999999999999e-2 * t1174 * t4934 * t24705 * t3469 + 0.29629629629629629628e-2 * t44499 - 0.22222222222222222221e-2 * t44502 + 0.34567901234567901234e-2 * t3447 * t44504 * t44506 + 0.11111111111111111111e-2 * t44512 + 0.33333333333333333332e-2 * t3447 * t11593 * t11584 + 0.11111111111111111111e-2 * t3447 * t44517 * t3451 + 0.11111111111111111111e-2 * t3447 * t44521 * t3451 - 0.14814814814814814814e-2 * t44527 - 0.22222222222222222222e-2 * t3447 * t44529 * t11571 - 0.99999999999999999996e-2 * t3447 * t4908 * t43719 + 0.66666666666666666664e-2 * t3447 * t3449 * t44536 + 0.11111111111111111111e-2 * t3447 * t3449 * t44540 - 0.22222222222222222221e-2 * t3447 * t4908 * t43723;
    (t44547,)
}
