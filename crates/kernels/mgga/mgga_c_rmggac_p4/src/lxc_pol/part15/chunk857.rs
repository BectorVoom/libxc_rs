//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 857/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk857<F: Float>(t9065: F, t9077: F, t9088: F, t9093: F, t9102: F, t9112: F, t9114: F, t9119: F, t9131: F, t9139: F, t9143: F, t9154: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42312 = F::cast_from(2.0_f64) * t9065;
    let t42313 = F::cast_from(0.39914139006212695214e-1_f64) * t9077;
    let t42316 = F::cast_from(0.85129199786595678796e-5_f64) * t9088;
    let t42317 = F::cast_from(0.39914139006212695214e-1_f64) * t9093;
    let t42320 = F::cast_from(0.11974241701863808564e0_f64) * t9102;
    let t42322 = F::cast_from(0.85129199786595678796e-5_f64) * t9112;
    let t42323 = F::cast_from(0.85129199786595678796e-5_f64) * t9114;
    let t42324 = F::cast_from(0.31923449919973379548e-4_f64) * t9119;
    let t42328 = F::cast_from(0.79828278012425390428e-1_f64) * t9131;
    let t42332 = F::cast_from(0.17025839957319135759e-4_f64) * t9139;
    let t42333 = F::cast_from(0.85129199786595678796e-5_f64) * t9143;
    let t42335 = F::cast_from(0.25538759935978703638e-4_f64) * t9154;
    (t42312, t42313, t42316, t42317, t42320, t42322, t42323, t42324, t42328, t42332, t42333, t42335)
}
