//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 347/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk347<F: Float>(t1133: F, t1141: F, t1142: F, t1143: F, t220: F, t468: F, t1139: F, t1134: F, t1136: F, t473: F, t475: F) -> (F, F, F, F) {
    let t1148 = t1133 * t220 * t468 + t1141 * t1142 * t1143;
    let t1149 = t1139 * t1148;
    let t1151 = t1134 * t473 - t1136 * t1149;
    let t1153 = F::cast_from(1.0_f64) / t475;
    (t1148, t1149, t1151, t1153)
}
