//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2120/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2120<F: Float>(t45844: F, t7245: F, t22550: F, t7974: F, t2109: F, t90247: F, t1419: F, t2274: F, t111: F, t27370: F, t2174: F, t5363: F) -> (F, F, F, F, F, F) {
    let t96120 = t45844 * t7245;
    let t96135 = t7974 * t22550;
    let t96138 = t2109 * t90247;
    let t96157 = t1419 * t2274;
    let t96238 = t27370 * t111;
    let t96281 = F::cast_from(2.0_f64) * t5363 * t2174;
    (t96120, t96135, t96138, t96157, t96238, t96281)
}
