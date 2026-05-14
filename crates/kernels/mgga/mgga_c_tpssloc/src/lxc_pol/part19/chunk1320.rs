//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1320/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1320<F: Float>(t1184: F, t44583: F, t3447: F, t3451: F, t11579: F, t11589: F, t11168: F, t15402: F, t11159: F, t15419: F, t11546: F, t11571: F, t11575: F, t11584: F, t11593: F, t1174: F, t3440: F, t3441: F, t39097: F, t39103: F, t43715: F, t44558: F, t44564: F, t44566: F, t44573: F, t44581: F, t4900: F) -> (F,) {
    let t44584 = t44583 * t1184;
    let t44586 = t3447 * t44584 * t3451;
    let t44589 = t3447 * t11589 * t11579;
    let t44592 = t3447 * t15402 * t11168;
    let t44595 = t3447 * t15419 * t11159;
    let t44600 = 0.16666666666666666666e-2 * t3447 * t11575 * t11579 + 0.33333333333333333332e-2 * t3447 * t11575 * t11584 + 0.16666666666666666666e-2 * t3447 * t11593 * t11579 - 0.22222222222222222222e-2 * t3447 * t44558 * t11571 - 0.11522633744855967078e-2 * t44564 - 0.1037037037037037037e-1 * t1174 * t11546 * t44566 * t39097 - 0.49382716049382716048e-3 * t44573 + 0.11111111111111111111e-2 * t1174 * t3440 * t3441 * t39103 + 0.11111111111111111111e-2 * t44581 - 0.74074074074074074072e-3 * t44586 + 0.11111111111111111111e-2 * t44589 - 0.22222222222222222221e-2 * t44592 + 0.14814814814814814815e-2 * t44595 + 0.14814814814814814815e-2 * t3447 * t4900 * t43715;
    (t44600,)
}
