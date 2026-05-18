//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 642/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk642<F: Float>(t31: F, t3966: F, t65: F, t1410: F, t628: F, t1426: F, t608: F, t1409: F, t2267: F, t607: F, t43: F, t2274: F) -> (F, F, F, F, F, F, F, F) {
    let t3967 = t31 * t3966;
    let t3968 = t3967 * t65;
    let t3971 = t1410 * t628;
    let t3976 = t608 * t1426;
    let t3981 = t2267 * t1409;
    let t3982 = t3981 * t607;
    let t3985 = t43 * t3966;
    let t3990 = t2274 * t1409;
    (t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990)
}
