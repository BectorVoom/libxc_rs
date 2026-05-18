//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1314/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1314<F: Float>(t110075: F, t30053: F, t29895: F, t30057: F, t29900: F, t30064: F, t9398: F, t99: F, t2196: F, t2585: F, t110140: F, t8181: F) -> (F, F, F, F, F, F) {
    let t110290 = t110075 * t30053;
    let t110292 = t29895 * t30057;
    let t110294 = t29900 * t30064;
    let t110314 = t99 * t9398;
    let t110333 = F::new(154.0) / F::new(27.0) * t2585 * t2196;
    let t110334 = t110140 * t8181;
    (t110290, t110292, t110294, t110314, t110333, t110334)
}
