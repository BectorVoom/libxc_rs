//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 820/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk820<F: Float>(t1998: F, t5318: F, t214: F, t1985: F, t7740: F, t794: F, t6897: F, t552: F, t6604: F, t1307: F, t26421: F, t26331: F) -> (F, F, F, F) {
    let t26432 = t1998 * t5318;
    let t26433 = t214 * t26432;
    let t26434 = t1985 * t26433;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    let t26446 = t6604 * t552;
    let t26447 = t26421 * t1307;
    let t26448 = t26446 * t26447;
    let t26449 = t26331 * t26448;
    (t26434, t26437, t26446, t26449)
}
