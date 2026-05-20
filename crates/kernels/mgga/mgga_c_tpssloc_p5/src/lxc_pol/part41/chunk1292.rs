//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1292/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1292<F: Float>(t103: F, t1453: F, t112: F, t30349: F, t111: F, t8283: F, t580: F, t1404: F, t1858: F, t8199: F, t2205: F, t5381: F) -> (F, F, F, F, F, F, F) {
    let t111134 = t103 * t1453;
    let t111226 = t30349 * t112;
    let t111246 = t8283 * t111;
    let t111289 = F::new(2.0) * t30349 * t580;
    let t111291 = F::new(2.0) * t8283 * t1404;
    let t111293 = F::new(2.0) * t8199 * t1858;
    let t111302 = F::new(2.0) * t2205 * t5381;
    (t111134, t111226, t111246, t111289, t111291, t111293, t111302)
}
