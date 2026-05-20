//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 402/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk402<F: Float>(t28: F, t1081: F, t1302: F, t1301: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t1305 = piecewise3::<F>(t29, F::new(0.0), F::new(2.0) / F::new(3.0) * t1302 * t1081);
    let t1307 = t1301 / F::new(2.0) + t1305 / F::new(2.0);
    t1307
}
