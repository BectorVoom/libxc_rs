//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 798/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk798<F: Float>(t109: F, t8319: F, t89: F, t510: F) -> (F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t8320 = t89 * t8319;
    let t8322 = F::new(2.0) * t8320 * t510;
    let t8326 = piecewise3::<f64>(t110, F::new(0.0), F::new(0.0));
    (t8320, t8322, t8326)
}
