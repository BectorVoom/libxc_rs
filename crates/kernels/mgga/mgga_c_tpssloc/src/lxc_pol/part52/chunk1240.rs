//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1240/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1240<F: Float>(t111: F, t26097: F, t1834: F, t794: F, t213: F, t225: F, t26219: F, t214: F, t5318: F, t1824: F, t6955: F, t2006: F, t5286: F) -> (F, F, F, F, F, F, F) {
    let t90400 = t26097 * t111;
    let t90544 = t794 * t1834;
    let t90566 = t213 * t1834 * t225;
    let t90732 = t26219 * t225;
    let t90739 = t214 * t5318;
    let t90942 = t6955 * t1824;
    let t90946 = t2006 * t5286;
    (t90400, t90544, t90566, t90732, t90739, t90942, t90946)
}
