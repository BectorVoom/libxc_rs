//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 814/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk814<F: Float>(t33277: F, t6888: F, t27074: F, t550: F, t6976: F, t1992: F, t1998: F, t7918: F, t214: F, t1985: F, t6906: F, t7936: F, t6889: F, t31611: F, t7691: F, t7700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33278 = t6888 * t33277;
    let t33280 = t27074 * t550;
    let t33281 = t6976 * t33280;
    let t33282 = t1992 * t33281;
    let t33284 = t1998 * t7918;
    let t33285 = t214 * t33284;
    let t33286 = t1985 * t33285;
    let t33296 = t6906 * t7936;
    let t33297 = t6889 * t33296;
    let t33298 = t1985 * t33297;
    let t33307 = t31611 * t7691;
    let t33308 = t6888 * t33307;
    let t33310 = t31611 * t7700;
    (t33278, t33280, t33281, t33282, t33284, t33285, t33286, t33296, t33297, t33298, t33307, t33308, t33310)
}
