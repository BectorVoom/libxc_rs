//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1864/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1864<F: Float>(t26193: F, t6891: F, t6888: F, t22674: F, t7691: F, t22892: F, t6883: F, t7701: F, t5353: F, t6906: F, t6889: F, t1985: F) -> (F, F, F, F, F, F, F, F) {
    let t26194 = t26193 * t6891;
    let t26195 = t6888 * t26194;
    let t26197 = t22674 * t7691;
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26202 = t6906 * t5353;
    let t26203 = t6889 * t26202;
    let t26204 = t1985 * t26203;
    (t26194, t26195, t26197, t26198, t26200, t26202, t26203, t26204)
}
