//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1050/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050<F: Float>(t535: F, t9534: F, t9538: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12215: F, t12217: F, t12222: F, t12228: F, t12231: F, t1315: F, t5195: F) -> F {
    let t12236 = F::cast_from(0.13888888888888888889e-3_f64) * t9534 * t535 * t9538;
    let t12237 = -t12188 - F::cast_from(0.38888888888888888888e-1_f64) * t12190 - t12194 + t12196 + F::cast_from(0.11666666666666666666e-1_f64) * t12197 - F::cast_from(0.15833333333333333333e-1_f64) * t12200 - F::cast_from(0.74999999999999999997e-2_f64) * t12205 + F::cast_from(0.24999999999999999999e-2_f64) * t12209 - F::cast_from(0.34999999999999999998e-1_f64) * t12212 - F::cast_from(0.19999999999999999999e-1_f64) * t12215 * t12217 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t12222 + F::cast_from(0.49999999999999999998e-2_f64) * t12228 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t12231 - t12236;
    t12237
}
