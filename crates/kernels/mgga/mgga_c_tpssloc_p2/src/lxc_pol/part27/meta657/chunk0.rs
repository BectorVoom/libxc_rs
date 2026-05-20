//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2297/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2297<F: Float>(t22633: F, t22635: F, t26214: F, t3719: F, t225: F, t26219: F, t1985: F, t7700: F, t80707: F, t214: F, t5318: F, t6888: F, t6891: F) -> (F, F, F, F, F) {
    let t90728 = t22633 * t22635 * t26214 * t3719;
    let t90732 = t26219 * t225;
    let t90737 = t1985 * t80707 * t7700;
    let t90739 = t214 * t5318;
    let t90741 = t6888 * t90739 * t6891;
    (t90728, t90732, t90737, t90739, t90741)
}
