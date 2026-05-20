//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2580/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2580<F: Float>(t1090: F, t11569: F, t1174: F, t1184: F, t15288: F, t15320: F, t15357: F, t15382: F, t15390: F, t24698: F, t3243: F, t3248: F, t3252: F, t3447: F, t3449: F, t3469: F, t44499: F, t44502: F, t44529: F, t460: F, t4908: F, t4919: F, t4928: F, t4934: F, t52216: F, t52220: F, t52224: F, t52228: F, t52236: F, t52240: F, t52250: F, t7319: F) -> F {
    let t52257 = F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t15320 * t15288 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t4919 * t24698 * t1090 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t4919 * t7319 * t3252 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4919 * t7319 * t3248 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t44529 * t15382 + F::cast_from(0.33333333333333333333e-2_f64) * t3447 * t3449 * t52216 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t3449 * t52220 + F::cast_from(0.49999999999999999999e-2_f64) * t3447 * t3449 * t52224 - F::cast_from(0.66666666666666666665e-2_f64) * t3447 * t11569 * t52228 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t15390 * t7319 * t3243 - F::cast_from(0.49999999999999999998e-2_f64) * t3447 * t4908 * t52236 + F::cast_from(0.14814814814814814815e-2_f64) * t52240 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t4934 * t15357 * t1184 * t460 + F::cast_from(0.74074074074074074072e-3_f64) * t44499 - F::cast_from(0.55555555555555555554e-3_f64) * t44502 - F::cast_from(0.16666666666666666666e-2_f64) * t52250 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t4934 * t4928 * t3469 * t460;
    t52257
}
