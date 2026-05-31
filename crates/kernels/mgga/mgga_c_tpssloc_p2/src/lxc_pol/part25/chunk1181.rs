//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1181/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1181<F: Float>(t23966: F, t9231: F, t6492: F, t22527: F, t23967: F, t22531: F, t22546: F, t22549: F, t23963: F, t23970: F, t605: F, t83722: F, t83745: F, t83778: F, t83820: F, t84174: F, t84180: F, t84183: F, t84186: F, t84190: F) -> F {
    let t84195 = t9231 * t23966;
    let t84196 = t84195 * t6492;
    let t84198 = t23967 * t22527;
    let t84200 = t23967 * t22531;
    let t84202 = -F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t84174 + F::cast_from(20.0_f64) * t83722 * t23970 + F::cast_from(10.0_f64) * t83778 * t23970 + F::cast_from(20.0_f64) * t22549 * t84180 + F::cast_from(10.0_f64) * t22549 * t84183 - F::cast_from(2.0_f64) * t605 * t84186 * t83820 + F::cast_from(30.0_f64) * t84190 * t22546 + F::cast_from(30.0_f64) * t23963 * t83745 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t84196 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t84198 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t84200;
    t84202
}
