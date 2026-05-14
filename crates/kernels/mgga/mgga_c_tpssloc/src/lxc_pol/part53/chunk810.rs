//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 810/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk810<F: Float>(t5248: F, t5249: F, t550: F, t31170: F, t1831: F, t8466: F, t1484: F, t1894: F, t59: F, t6591: F, t1510: F, t6612: F, t6605: F, t1499: F, t8342: F, t8344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32721 = t5248 * t5249 * t550;
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32834 = t1894 * t59 * t1484;
    let t32835 = t6591 * t32834;
    let t32837 = t6612 * t1510;
    let t32838 = t6605 * t32837;
    let t32840 = t1499 * t8342;
    let t32841 = t32840 * t8344;
    (t32721, t32722, t32724, t32834, t32835, t32837, t32838, t32840, t32841)
}
