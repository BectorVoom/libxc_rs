//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2036/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2036<F: Float>(t102386: F, t1266: F, t1393: F, t19461: F, t2040: F, t2075: F, t2314: F, t24432: F, t24995: F, t26161: F, t26558: F, t26872: F, t26878: F, t26880: F, t27171: F, t28030: F, t28943: F, t28951: F, t28952: F, t29219: F, t29241: F, t29380: F, t4028: F, t4034: F, t5457: F, t652: F, t672: F, t6876: F, t7050: F, t7156: F, t75210: F, t7685: F, t91655: F, t96709: F, t97902: F, t97933: F) -> F {
    let t103029 = -F::cast_from(4.0_f64) * t4028 * t27171 - F::cast_from(2.0_f64) * t96709 * t2040 - F::cast_from(2.0_f64) * t97933 * t2040 - F::cast_from(2.0_f64) * t28030 * t7050 - F::cast_from(12.0_f64) * t24995 * t24432 * t97902 + t29241 * t1393 - F::cast_from(2.0_f64) * t102386 * t672 - F::cast_from(4.0_f64) * t2314 * t29219 + F::cast_from(6.0_f64) * t6876 * t29380 - F::cast_from(2.0_f64) * t7685 * t26880 - F::cast_from(2.0_f64) * t7685 * t26878 + F::cast_from(2.0_f64) * t26161 * t26558 * t75210 - t28943 * t1266 - F::cast_from(2.0_f64) * t19461 * t2075 - F::cast_from(2.0_f64) * t5457 * t7156 - F::cast_from(6.0_f64) * t91655 * t26872 - F::cast_from(2.0_f64) * t2314 * t28952 - F::cast_from(2.0_f64) * t4034 * t28952 - F::cast_from(2.0_f64) * t652 * t1266 * t28951;
    t103029
}
