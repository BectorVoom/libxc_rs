//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 788/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk788<F: Float>(t1992: F, t31628: F, t1998: F, t7191: F, t214: F, t1985: F, t31611: F, t6891: F, t6888: F, t6883: F, t8622: F, t22666: F, t8621: F, t8612: F, t225: F, t8729: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31629 = t1992 * t31628;
    let t31631 = t1998 * t7191;
    let t31632 = t214 * t31631;
    let t31633 = t1985 * t31632;
    let t31645 = t31611 * t6891;
    let t31646 = t6888 * t31645;
    let t31648 = t6883 * t8622;
    let t31650 = t22666 * t8621;
    let t31651 = t1985 * t31650;
    let t31662 = t6883 * t8612;
    let t31964 = t8729 * t225;
    (t31629, t31631, t31632, t31633, t31645, t31646, t31648, t31650, t31651, t31662, t31964)
}
