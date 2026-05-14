//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 728/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk728<F: Float>(t22892: F, t26197: F, t6883: F, t7701: F, t5353: F, t6906: F, t6889: F, t1985: F, t26193: F, t6907: F, t225: F, t5318: F, t567: F, t214: F, t1377: F, t1842: F) -> (F, F, F, F, F, F, F) {
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26202 = t6906 * t5353;
    let t26203 = t6889 * t26202;
    let t26204 = t1985 * t26203;
    let t26206 = t26193 * t6907;
    let t26207 = t1985 * t26206;
    let t26210 = t5318 * t225 * t567;
    let t26211 = t214 * t26210;
    let t26212 = t1985 * t26211;
    let t26214 = t1377 * t1842;
    (t26198, t26200, t26202, t26204, t26207, t26212, t26214)
}
