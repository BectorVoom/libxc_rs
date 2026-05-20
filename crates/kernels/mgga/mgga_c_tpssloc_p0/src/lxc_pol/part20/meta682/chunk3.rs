//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2577/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2577<F: Float>(t3447: F, t44579: F, t4904: F, t11545: F, t134: F, t461: F, t14726: F, t11579: F, t15338: F, t4899: F, t4928: F, t11563: F, t11571: F, t11572: F, t11575: F, t15313: F, t15376: F, t15390: F, t15395: F, t44506: F, t44521: F, t44608: F, t4908: F, t50865: F, t50869: F, t50910: F, t50924: F, t52096: F, t52100: F, t52110: F, t52122: F, t52124: F) -> F {
    let t52127 = t3447 * t44579 * t4904;
    let t52133 = t134 * t11545 * t461;
    let t52135 = t3447 * t52133 * t14726;
    let t52138 = t3447 * t15338 * t11579;
    let t52140 = t4899 * t4928;
    let t52150 = F::cast_from(0.28806584362139917695e-2_f64) * t3447 * t52096 * t50924 + F::cast_from(0.86419753086419753084e-3_f64) * t3447 * t52100 * t44506 + F::cast_from(0.44444444444444444445e-2_f64) * t15376 * t11563 - t52110 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t11575 * t15313 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t50865 - F::cast_from(0.49999999999999999999e-2_f64) * t3447 * t4908 * t50869 + F::cast_from(0.2962962962962962963e-2_f64) * t15376 * t11572 - F::cast_from(0.14814814814814814815e-2_f64) * t52122 - F::cast_from(0.82304526748971193415e-3_f64) * t52124 + F::cast_from(0.27777777777777777777e-3_f64) * t52127 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t44521 * t4904 - F::cast_from(0.86419753086419753084e-3_f64) * t52135 + F::cast_from(0.27777777777777777777e-3_f64) * t52138 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t52140 * t11571 - F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t15390 * t44608 - F::cast_from(0.25925925925925925925e-2_f64) * t3447 * t15395 * t50910;
    t52150
}
