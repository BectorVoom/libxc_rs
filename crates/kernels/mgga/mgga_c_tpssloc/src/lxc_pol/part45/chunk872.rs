//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 872/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk872<F: Float>(t2047: F, t213: F, t225: F, t22986: F, t23272: F, t1880: F, t82124: F, t8547: F, t31351: F, t794: F, t6562: F, t6572: F, t82133: F, t7106: F, t857: F, t23270: F, t776: F) -> (F, F, F, F, F, F, F) {
    let t114770 = t213 * t2047 * t225;
    let t114772 = t22986 * t114770 * t23272;
    let t114781 = t1880 * t82124 * t8547;
    let t114785 = t31351 * t225;
    let t114790 = t794 * t2047;
    let t114792 = t6562 * t114790 * t6572;
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114800 = t22986 * t23270 * t114797 * t776;
    (t114772, t114781, t114785, t114790, t114792, t114795, t114800)
}
