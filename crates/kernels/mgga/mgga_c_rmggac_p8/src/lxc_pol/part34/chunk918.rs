//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 918/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk918<F: Float>(t70545: F, t14102: F, t8365: F, t638: F, t639: F, t640: F, t9030: F, t2046: F, t3047: F, t8850: F, t8854: F, t36292: F, t739: F, t8936: F) -> (F, F, F, F, F, F) {
    let t76515 = F::cast_from(0.79828278012425390427e-1_f64) * t70545;
    let t76517 = t8365 * t14102;
    let t76521 = t638 * t639 * t640 * t9030;
    let t76524 = t2046 * t3047 * t8850;
    let t76527 = t2046 * t3047 * t8854;
    let t76538 = t739 * t36292 * t8936;
    (t76515, t76517, t76521, t76524, t76527, t76538)
}
