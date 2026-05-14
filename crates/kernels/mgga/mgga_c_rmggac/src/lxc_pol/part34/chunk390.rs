//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 390/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk390<F: Float>(t1969: F, t8576: F, t128: F, t1528: F, t118: F, t446: F, t597: F, t201: F, t1451: F, t194: F, t1525: F, t22: F, t7262: F, t235: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8577 = t8576 * t1969;
    let t8580 = t128 * t1528;
    let t8581 = t118 * t8580;
    let t8601 = t446 * t597;
    let t8602 = t8601 * t201;
    let t8607 = t194 * t1451;
    let t8608 = t8607 * t201;
    let t8614 = t128 * t1525;
    let t8615 = t118 * t8614;
    let t8619 = t7262 * t22;
    let t8620 = t235 * t8619;
    (t8577, t8580, t8581, t8601, t8602, t8607, t8608, t8614, t8615, t8619, t8620)
}
