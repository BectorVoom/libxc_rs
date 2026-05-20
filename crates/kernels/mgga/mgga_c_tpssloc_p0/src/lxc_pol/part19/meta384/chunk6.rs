//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1441/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1441<F: Float>(t11126: F, t3419: F, t11478: F, t3411: F, t3633: F, t3415: F, t1164: F, t3400: F, t3403: F, t44168: F, t1156: F, t3375: F) -> (F, F, F, F, F, F) {
    let t44375 = F::cast_from(0.35089341735807877242e1_f64) * t11126 * t3419;
    let t44377 = F::cast_from(0.23392894490538584828e1_f64) * t3411 * t11478;
    let t44378 = t3633 * t3633;
    let t44384 = F::cast_from(0.70178683471615754484e1_f64) * t11126 * t3415;
    let t44388 = F::cast_from(0.51947577317044391277e2_f64) * t1164 * t3400 * t44168 * t3403;
    let t44392 = F::cast_from(0.35089341735807877242e1_f64) * t1164 * t3375 * t44168 * t1156;
    (t44375, t44377, t44378, t44384, t44388, t44392)
}
