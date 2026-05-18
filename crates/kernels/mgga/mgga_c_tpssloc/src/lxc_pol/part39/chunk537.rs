//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 537/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk537<F: Float>(t103: F, t2354: F, t100: F, t2336: F, t2343: F, t2346: F, t2351: F, t657: F, t660: F, t92: F, t96: F) -> (F, F) {
    let t2355 = t103 * t2354;
    let t2358 = F::new(40.0) / F::new(9.0) * t2336 * t96 - F::new(50.0) / F::new(9.0) * t657 * t660 + F::new(10.0) / F::new(9.0) * t92 * t2343 + F::new(5.0) / F::new(3.0) * t92 * t2346 + F::new(10.0) / F::new(9.0) * t100 * t2351 + F::new(5.0) / F::new(3.0) * t100 * t2355;
    (t2355, t2358)
}
