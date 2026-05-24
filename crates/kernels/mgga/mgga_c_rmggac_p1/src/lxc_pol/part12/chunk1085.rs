//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1085/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1085<F: Float>(t9131: F, t9139: F, t9143: F, t9154: F, t9160: F, t9166: F, t9172: F, t9174: F, t9176: F, t9178: F, t7937: F, t7946: F, t8304: F, t9653: F) -> (F, F, F, F) {
    let t42328 = F::cast_from(0.79828278012425390428e-1_f64) * t9131;
    let t42332 = F::cast_from(0.17025839957319135759e-4_f64) * t9139;
    let t42333 = F::cast_from(0.85129199786595678796e-5_f64) * t9143;
    let t42335 = F::cast_from(0.25538759935978703638e-4_f64) * t9154;
    let t42336 = F::cast_from(0.25538759935978703638e-4_f64) * t9160;
    let t42337 = F::cast_from(0.85129199786595678796e-5_f64) * t9166;
    let t42338 = F::cast_from(0.85129199786595678796e-5_f64) * t9172;
    let t42339 = F::cast_from(0.11974241701863808564e0_f64) * t9174;
    let t42340 = F::cast_from(0.11974241701863808564e0_f64) * t9176;
    let t42341 = F::cast_from(0.79828278012425390428e-1_f64) * t9178;
    let t42343 = t9653 - t42335 + t42336 + t42337 + t42338 - t42339 - t42340 + t42341 + F::cast_from(0.14408463291498358381e-2_f64) * t7937 - t8304 + t7946;
    (t42328, t42332, t42333, t42343)
}
