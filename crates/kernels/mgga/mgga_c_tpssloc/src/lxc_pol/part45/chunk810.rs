//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 810/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk810<F: Float>(t109: F, t2039: F, t3652: F, t22468: F, t22471: F, t22474: F, t22476: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t23909 = t3652 * t2039;
    let t23912 = F::new(22.0) / F::new(9.0) * t22468;
    let t23917 = piecewise3::<f64>(t110, F::new(0.0), t23912 + F::new(4.0) / F::new(3.0) * t22471 + t22474 / F::new(2.0) - t22476 / F::new(4.0));
    (t23909, t23917)
}
