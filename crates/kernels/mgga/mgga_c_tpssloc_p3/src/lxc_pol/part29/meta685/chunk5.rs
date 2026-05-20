//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2343/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343<F: Float>(t2109: F, t90090: F, t90094: F, t45844: F, t7245: F, t22546: F, t22549: F, t24514: F, t24517: F, t26016: F, t7432: F, t85470: F, t85473: F, t85476: F, t85507: F, t90072: F, t90076: F, t90098: F, t90101: F, t90104: F) -> F {
    let t96110 = t2109 * t90090;
    let t96115 = t2109 * t90094;
    let t96120 = t45844 * t7245;
    let t96133 = -F::new(5.0) / F::new(3.0) * t26016 * t85476 - F::new(10.0) * t24514 * t90072 - F::new(10.0) / F::new(3.0) * t22549 * t96110 - F::new(10.0) * t24514 * t90076 - F::new(10.0) / F::new(3.0) * t22549 * t96115 - F::new(5.0) / F::new(3.0) * t85507 * t7432 - F::new(5.0) * t96120 * t22546 - F::new(10.0) / F::new(3.0) * t90098 * t24517 - F::new(10.0) / F::new(3.0) * t90101 * t24517 - F::new(10.0) / F::new(3.0) * t90104 * t24517 - F::new(10.0) / F::new(3.0) * t26016 * t85470 - F::new(10.0) / F::new(3.0) * t26016 * t85473;
    t96133
}
