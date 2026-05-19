//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 894/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk894<F: Float>(t109: F, t662: F, t8184: F, t8128: F, t8137: F, t8179: F, t8181: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t8185 = t8184 * t662;
    let t8189 = piecewise3::<F>(t110, F::new(0.0), t8179 + t8128 * t8181 / F::new(4.0) - F::new(5.0) / F::new(24.0) * t8137 * t8185);
    (t8185, t8189)
}
