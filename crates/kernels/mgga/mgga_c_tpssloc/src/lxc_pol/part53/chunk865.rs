//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 865/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk865<F: Float>(t114011: F, t32721: F, t12419: F, t1307: F, t1336: F, t240: F, t241: F, t5301: F, t552: F, t1824: F, t22705: F, t22852: F, t550: F, t59: F, t22827: F, t26297: F, t6943: F) -> (F, F, F, F) {
    let t120350 = t114011 * t32721;
    let t120357 = t1336 * t552 * t240 * t241 * t12419 * t5301 * t1307;
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    let t120366 = t22827 * t6943 * t26297;
    (t120350, t120357, t120363, t120366)
}
