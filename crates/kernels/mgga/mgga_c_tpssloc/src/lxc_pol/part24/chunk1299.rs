//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1299/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1299<F: Float>(t109: F, t81438: F, t81440: F, t81443: F, t81445: F, t81447: F, t81450: F, t81452: F, t510: F, t652: F, t1983: F, t22584: F, t22591: F) -> (F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t81455 = piecewise3::<F>(t110, F::new(0.0), -t81438 - F::new(11.0) / F::new(3.0) * t81440 - F::new(2.0) * t81443 + t81445 - F::new(3.0) / F::new(4.0) * t81447 + F::new(3.0) / F::new(4.0) * t81450 - t81452 / F::new(8.0));
    let t81458 = F::new(2.0) * t652 * t510 * t81455;
    let t81469 = F::new(9.0) * t1983 * t22591 * t22584;
    (t81455, t81458, t81469)
}
