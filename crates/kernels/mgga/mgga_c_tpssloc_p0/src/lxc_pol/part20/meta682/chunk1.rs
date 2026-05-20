//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2575/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2575<F: Float>(t14749: F, t15402: F, t3447: F, t11499: F, t11505: F, t44424: F, t44439: F, t44504: F, t4889: F, t52050: F, t52053: F, t52058: F, t52061: F, t52064: F, t52066: F, t52074: F, t52076: F, t52081: F, t52085: F, t52086: F, t52089: F) -> F {
    let t52092 = t3447 * t15402 * t14749;
    let t52094 = F::cast_from(0.37037037037037037036e-3_f64) * t52050 + F::cast_from(0.55555555555555555554e-3_f64) * t52053 + t52058 - F::cast_from(0.37037037037037037036e-3_f64) * t52061 + F::cast_from(0.74074074074074074072e-3_f64) * t52064 + F::cast_from(0.25925925925925925925e-2_f64) * t3447 * t44504 * t52066 + F::cast_from(0.55555555555555555554e-3_f64) * t44424 + F::cast_from(0.55555555555555555554e-3_f64) * t44439 + F::cast_from(0.22222222222222222222e-2_f64) * t4889 * t11505 - F::cast_from(0.14814814814814814815e-2_f64) * t52074 + F::cast_from(0.22222222222222222222e-2_f64) * t52076 + F::cast_from(0.22222222222222222222e-2_f64) * t4889 * t11499 - F::cast_from(0.3086419753086419753e-3_f64) * t52081 + t52085 + F::cast_from(0.22222222222222222222e-2_f64) * t52086 + F::cast_from(0.55555555555555555554e-3_f64) * t52089 - F::cast_from(0.11111111111111111111e-2_f64) * t52092;
    t52094
}
