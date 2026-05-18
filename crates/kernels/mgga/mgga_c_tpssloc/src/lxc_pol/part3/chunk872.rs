//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 872/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk872<F: Float>(t2409: F, t681: F, t125: F, t141: F, t2413: F, t9454: F) -> F {
    let t9478 = F::new(1.0) / t2409 / t681;
    let t9479 = t125 * t9478;
    let t9481 = F::new(1.0) / t2413 / t141;
    let t9482 = t9454 * t9481;
    let t9484 = F::new(0.51726012919273400301e3) * t9479 * t9482;
    t9484
}
