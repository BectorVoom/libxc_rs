//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 898/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk898<F: Float>(t1268: F, t2200: F, t2202: F, t2314: F, t4034: F, t5113: F, t652: F, t8176: F, t8190: F, t8194: F, t8196: F) -> F {
    let t8199 = F::new(2.0) * t1268 * t8194 + F::new(2.0) * t1268 * t8196 - F::new(2.0) * t2200 * t2314 - F::new(2.0) * t2200 * t4034 + F::new(2.0) * t2202 * t2314 + F::new(2.0) * t2202 * t5113 - F::new(2.0) * t652 * t8176 - F::new(2.0) * t652 * t8190;
    t8199
}
