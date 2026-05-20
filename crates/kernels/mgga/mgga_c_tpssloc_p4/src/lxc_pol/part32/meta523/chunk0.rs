//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1857/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1857<F: Float>(t26223: F, t26364: F, t26485: F, t26500: F, t533: F, t1390: F, t1983: F, t16521: F, t1873: F, t16524: F, t7015: F, t5371: F, t6534: F) -> (F, F, F, F, F, F, F) {
    let t26502 = t26223 + t26364 + t26485 + t26500;
    let t26503 = t533 * t26502;
    let t26504 = t26503 * t1390;
    let t26505 = t1983 * t26504;
    let t26533 = F::new(0.135e2) * t16521 * t1873;
    let t26535 = F::new(27.0) * t16524 * t7015;
    let t26537 = F::new(0.135e2) * t5371 * t6534;
    (t26502, t26503, t26504, t26505, t26533, t26535, t26537)
}
