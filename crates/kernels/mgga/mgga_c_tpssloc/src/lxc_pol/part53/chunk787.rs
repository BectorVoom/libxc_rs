//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 787/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk787<F: Float>(t31611: F, t6907: F, t1985: F, t6883: F, t8631: F, t2085: F, t552: F, t1307: F, t6637: F, t6888: F, t794: F, t8630: F, t6897: F, t1351: F, t550: F, t6976: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31612 = t31611 * t6907;
    let t31613 = t1985 * t31612;
    let t31616 = t6883 * t8631;
    let t31618 = t552 * t2085;
    let t31619 = t31618 * t1307;
    let t31620 = t6637 * t31619;
    let t31621 = t6888 * t31620;
    let t31623 = t794 * t8630;
    let t31624 = t6897 * t31623;
    let t31627 = t2085 * t1351 * t550;
    let t31628 = t6976 * t31627;
    (t31612, t31613, t31616, t31618, t31619, t31620, t31621, t31623, t31624, t31627, t31628)
}
