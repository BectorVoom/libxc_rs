//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 664/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk664<F: Float>(t109: F, t2358: F, t656: F, t2327: F, t2328: F, t2333: F, t64: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t2359 = t656 * t2358;
    let t2363 = piecewise3::<F>(t110, F::new(0.0), t2327 + F::new(2.0) / F::new(3.0) * t2328 + t64 * t2333 / F::new(4.0) - t64 * t2359 / F::new(8.0));
    (t2359, t2363)
}
