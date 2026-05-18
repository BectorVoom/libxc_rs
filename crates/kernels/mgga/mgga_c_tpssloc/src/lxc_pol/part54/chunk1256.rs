//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1256/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1256<F: Float>(t1824: F, t6955: F, t2006: F, t5286: F, t225: F, t26221: F, t26329: F, t26229: F, t1324: F, t254: F, t22573: F, t7684: F) -> (F, F, F, F, F, F, F) {
    let t90942 = t6955 * t1824;
    let t90946 = t2006 * t5286;
    let t91441 = t26221 * t225;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91505 = t1324 * t254;
    let t91655 = t7684 * t22573;
    (t90942, t90946, t91441, t91488, t91491, t91505, t91655)
}
