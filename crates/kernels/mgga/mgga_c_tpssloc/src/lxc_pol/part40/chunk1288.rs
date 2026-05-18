//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1288/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1288<F: Float>(t1453: F, t95: F, t2331: F, t64: F, t91: F, t29900: F, t30168: F, t656: F, t9576: F, t30176: F, t29895: F, t30159: F) -> (F, F, F, F, F) {
    let t110521 = t95 * t1453;
    let t110526 = t64 * t2331 * t91;
    let t110531 = F::new(50.0) / F::new(27.0) * t29900 * t30168;
    let t110532 = t9576 * t656;
    let t110533 = t110532 * t30176;
    let t110542 = F::new(4.0) / F::new(3.0) * t29895 * t30159;
    (t110521, t110526, t110531, t110533, t110542)
}
