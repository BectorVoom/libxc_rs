//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1152/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1152<F: Float>(t64: F, t9365: F, t2196: F, t2281: F, t29895: F, t8181: F, t29900: F, t8185: F, t2349: F, t99: F) -> (F, F, F, F, F) {
    let t29903 = t64 * t9365;
    let t30048 = 11.0 / 9.0 * t2281 * t2196;
    let t30049 = t29895 * t8181;
    let t30051 = t29900 * t8185;
    let t30063 = t99 * t2349;
    (t29903, t30048, t30049, t30051, t30063)
}
