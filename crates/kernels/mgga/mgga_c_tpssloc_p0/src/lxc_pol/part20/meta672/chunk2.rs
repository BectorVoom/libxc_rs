//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2528/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2528<F: Float>(t51034: F, t51037: F, t51039: F, t51041: F, t51043: F, t51046: F, t51049: F, t51051: F, t51053: F, t51056: F, t51100: F, t51102: F) -> F {
    let t51239 = F::cast_from(0.10954222222222222222e0_f64) * t51034 - F::cast_from(0.49293999999999999999e0_f64) * t51037 + F::cast_from(0.5477111111111111111e0_f64) * t51039 - F::cast_from(0.32862666666666666666e0_f64) * t51041 - F::cast_from(0.98587999999999999998e0_f64) * t51043 - F::cast_from(0.82156666666666666668e-1_f64) * t51046 - F::new(0.49294e0) * t51049 - F::cast_from(0.91285185185185185185e-1_f64) * t51051 - F::cast_from(0.65725333333333333332e0_f64) * t51053 + F::cast_from(0.49293999999999999999e0_f64) * t51056 + F::new(0.1898925e1) * t51100 + F::new(0.3071625e0) * t51102;
    t51239
}
