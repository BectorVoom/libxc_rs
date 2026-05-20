//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1259/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1259<F: Float>(t109: F, t63: F, t9365: F, t9366: F, t2358: F, t666: F, t22473: F, t6530: F, t9411: F, t81438: F, t81440: F, t81443: F, t81445: F) -> F {
    let t110 = F::new(1.0) < t109;
    let t81446 = t63 * t9365;
    let t81447 = t81446 * t9366;
    let t81449 = t666 * t2358;
    let t81450 = t22473 * t81449;
    let t81452 = t6530 * t9411;
    let t81455 = piecewise3::<F>(t110, F::new(0.0), -t81438 - F::new(11.0) / F::new(3.0) * t81440 - F::new(2.0) * t81443 + t81445 - F::new(3.0) / F::new(4.0) * t81447 + F::new(3.0) / F::new(4.0) * t81450 - t81452 / F::new(8.0));
    t81455
}
