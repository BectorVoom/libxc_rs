//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 977/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk977<F: Float>(t57: F, t1289: F, t8061: F, t2232: F, t3431: F, t10353: F, t1985: F, t1992: F, t3582: F, t581: F, t81: F, t10484: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t155 = t57 <= zeta_threshold;
    let t10485 = t8061 * t1289;
    let t10488 = t2232 * t3431;
    let t10496 = piecewise3::<f64>(t155, F::new(0.0), F::new(8.0) / F::new(27.0) * t10485 * t1985 + F::new(8.0) / F::new(9.0) * t10488 * t581 + F::new(4.0) / F::new(9.0) * t3582 * t1992 - F::new(4.0) / F::new(3.0) * t81 * t10353);
    let t10497 = t10484 + t10496;
    let t10498 = t10497 * t162;
    (t10497, t10498)
}
