//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 548/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk548<F: Float>(t31: F, t3966: F, t65: F, t1410: F, t628: F, t1426: F, t608: F, t1409: F, t2267: F, t607: F, t43: F, t2274: F, t55: F, t1414: F, t1420: F, t2282: F, t39: F, t51: F, t615: F, t621: F) -> (F, F, F, F, F) {
    let t3967 = t31 * t3966;
    let t3968 = t3967 * t65;
    let t3971 = t1410 * t628;
    let t3976 = t608 * t1426;
    let t3981 = t2267 * t1409;
    let t3982 = t3981 * t607;
    let t3985 = t43 * t3966;
    let t3990 = t2274 * t1409;
    let t3991 = t3990 * t607;
    let t3994 = t55 * t3966;
    let t3997 = -20.0 / 9.0 * t615 * t1414 + 5.0 / 18.0 * t39 * t3982 + 5.0 / 6.0 * t39 * t3985 + 20.0 / 9.0 * t1420 * t621 + 5.0 / 18.0 * t51 * t3991 - 5.0 / 6.0 * t51 * t3994 - t2282;
    (t3967, t3968, t3971, t3976, t3997)
}
