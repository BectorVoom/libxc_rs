//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 640/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk640<F: Float>(t2022: F, t3: F, t1401: F, t1873: F, t577: F, t11: F, t2: F, t584: F) -> (F, F, F, F) {
    let t2023 = t3 * t2022;
    let t2028 = F::new(0.135e2) * t1401 * t1873;
    let t2029 = F::new(0.45e1) * t2022 * t577 + t2028;
    let t2218 = F::new(0.174e1) * t11;
    let t2219 = t2 * t584;
    (t2023, t2029, t2218, t2219)
}
