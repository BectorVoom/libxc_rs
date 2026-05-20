//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 638/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk638<F: Float>(t2169: F, t3: F, t2028: F, t577: F, t11: F, t2: F, t584: F) -> (F, F, F, F) {
    let t2170 = t3 * t2169;
    let t2174 = F::new(0.45e1) * t2169 * t577 + t2028;
    let t2218 = F::new(0.174e1) * t11;
    let t2219 = t2 * t584;
    (t2170, t2174, t2218, t2219)
}
