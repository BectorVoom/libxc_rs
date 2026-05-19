//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 705/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk705<F: Float>(t201: F, t14056: F, t14371: F, t13889: F, t14368: F, t13892: F, t14007: F, t26004: F, t3051: F, t3052: F, t1343: F, t69097: F, t69101: F, t71: F) -> (F, F, F, F, F, F) {
    let t69635 = t201 * t201;
    let t69648 = t14371 * t14056;
    let t69662 = t14368 * t13889;
    let t69663 = F::cast_from(0.16351352353374609375e-5_f64) * t69662;
    let t69664 = t14368 * t13892;
    let t69666 = t14368 * t14007;
    let t69670 = t3051 / t3052 / t26004;
    let t69674 = t69670 * t69097 * t1343 * t71 * t69101;
    (t69635, t69648, t69663, t69664, t69666, t69674)
}
