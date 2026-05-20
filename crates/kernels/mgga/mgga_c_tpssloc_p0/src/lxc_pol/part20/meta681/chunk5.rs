//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2572/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2572<F: Float>(t3447: F, t3451: F, t52036: F, t15357: F, t3448: F, t11579: F, t11584: F, t11593: F, t15313: F, t15320: F, t15382: F, t44517: F, t44536: F, t44540: F, t44558: F, t4900: F, t4904: F, t4908: F, t4919: F, t50857: F, t50861: F, t50873: F, t50964: F, t51995: F, t52013: F, t52019: F, t52022: F) -> F {
    let t52038 = t3447 * t52036 * t3451;
    let t52040 = t3448 * t15357;
    let t52047 = -F::cast_from(0.16666666666666666666e-2_f64) * t51995 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t15320 * t11579 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t15320 * t11584 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t4919 * t44540 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4919 * t44536 - F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4908 * t50873 - F::cast_from(0.83333333333333333331e-3_f64) * t52013 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t44558 * t15382 + F::cast_from(0.11111111111111111111e-2_f64) * t52019 - F::cast_from(0.74074074074074074072e-3_f64) * t52022 + F::cast_from(0.66666666666666666665e-2_f64) * t3447 * t4900 * t50964 + F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t4900 * t50857 + F::cast_from(0.13333333333333333332e-1_f64) * t3447 * t4900 * t50861 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t44517 * t4904 + F::cast_from(0.55555555555555555554e-3_f64) * t52038 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t52040 * t3451 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t11593 * t15313;
    t52047
}
