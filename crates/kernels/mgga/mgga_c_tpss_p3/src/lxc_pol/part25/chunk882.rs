//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 882/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk882<F: Float>(t735: F, t8021: F, t2332: F, t692: F, t2210: F, t720: F, t7813: F, t256: F, t750: F, t7875: F, t7878: F, t200: F, t45: F) -> (F, F, F, F, F, F, F, F) {
    let t8023 = F::cast_from(0.10389515463408878255e3_f64) * t735 * t8021;
    let t8024 = t692 * t2332;
    let t8027 = t2210 * t7813 * t720;
    let t8029 = F::cast_from(0.35089341735807877242e1_f64) * t735 * t8027;
    let t8030 = t256 * t750;
    let t8038 = t7875 * t7813 * t7878;
    let t8040 = F::cast_from(0.10254018858216406658e4_f64) * t735 * t8038;
    let t8050 = F::cast_from(1.0_f64) / t200 / t45;
    (t8023, t8024, t8027, t8029, t8030, t8038, t8040, t8050)
}
