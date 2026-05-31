//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 466/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk466<F: Float>(t138: F, t681: F, t125: F, t2412: F, t702: F) -> F {
    let t2418 = t681 * t138;
    let t2419 = F::cast_from(1.0_f64) / t2418;
    let t2420 = t125 * t2419;
    let t2421 = t2412 * t702;
    let t2423 = F::cast_from(2.0_f64) * t2420 * t2421;
    t2423
}
