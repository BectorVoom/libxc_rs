//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2164/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2164<F: Float>(t25373: F, t57921: F, t1530: F, t2249: F, t16596: F, t81547: F, t1877: F, t1915: F, t22951: F, t22959: F, t22968: F, t23295: F, t23296: F, t23302: F, t25013: F, t2522: F, t25354: F, t25358: F, t4314: F, t606: F, t6542: F, t6670: F, t7541: F, t87953: F, t87957: F, t87961: F, t87975: F, t87978: F, t87981: F, t87984: F) -> F {
    let t87988 = t25373 * t57921;
    let t87994 = t2249 * t1530;
    let t87998 = t81547 * t16596;
    let t88001 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t87953 + F::cast_from(3.0_f64) * t2522 * t1915 * t87957 + t1877 * t23295 * t87961 + t1877 * t25354 * t606 + F::cast_from(3.0_f64) * t2522 * t25354 * t6542 - t1877 * t25358 * t23302 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7541 * t22968 + t1877 * t87975 * t23296 + F::cast_from(6.0_f64) * t25013 * t87978 + F::cast_from(3.0_f64) * t25013 * t87981 - t1877 * t6670 * t87984 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t22959 * t87988 + F::cast_from(3.0_f64) * t4314 * t7541 * t22951 - t1877 * t6670 * t87994 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t22959 * t87998;
    t88001
}
