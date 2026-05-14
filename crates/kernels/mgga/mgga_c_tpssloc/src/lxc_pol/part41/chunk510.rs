//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 510/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk510<F: Float>(t2205: F, t3: F, t1401: F, t2199: F, t577: F, t11: F, t2: F, t584: F) -> (F, F, F, F) {
    let t2206 = t3 * t2205;
    let t2212 = 0.45e1 * t2205 * t577 + 0.135e2 * t1401 * t2199;
    let t2218 = 0.174e1 * t11;
    let t2219 = t2 * t584;
    (t2206, t2212, t2218, t2219)
}
