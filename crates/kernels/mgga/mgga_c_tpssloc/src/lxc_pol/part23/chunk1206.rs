//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1206/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1206<F: Float>(t41209: F, t41212: F, t46806: F, t59195: F, t59204: F, t59206: F, t59218: F, t59221: F, t59224: F, t68116: F, t68118: F, t68122: F, t68131: F, t76359: F, t225: F, t13222: F, t13228: F, t1512: F, t20953: F, t237: F, t249: F, t4167: F, t4178: F, t59259: F, t59263: F, t59276: F, t59288: F, t67872: F, t68148: F, t68195: F, t68197: F, t68199: F, t68201: F, t76250: F) -> (F, F, F) {
    let t76371 = 0.11111111111111111111e-2 * t46806 - 0.77777777777777777775e-1 * t59195 + 0.15555555555555555555e-1 * t68116 + 0.18666666666666666665e0 * t68118 + 0.39999999999999999998e-1 * t68122 + 0.33333333333333333332e-2 * t68131 + t41209 + t41212 + 0.23333333333333333332e0 * t59204 + 0.94999999999999999997e-1 * t59206 - 0.31666666666666666666e-1 * t59218 - 0.29999999999999999998e-1 * t59221 + 0.99999999999999999996e-2 * t59224;
    let t76372 = t76359 + t76371;
    let t76373 = t76372 * t225;
    let t76394 = t76373 * t237 * t249 / 3072.0 - 7.0 / 4.0 * t68148 - 119.0 / 288.0 * t59259 - 119.0 / 576.0 * t59263 - t4167 * t20953 / 768.0 - t67872 * t1512 / 768.0 - 119.0 / 2304.0 * t59276 + 119.0 / 2304.0 * t59288 + 35.0 / 48.0 * t68195 - 35.0 / 96.0 * t68197 + 7.0 / 96.0 * t68199 + 7.0 / 96.0 * t68201 - t4178 * t13222 * t13228 * t76250 / 32.0;
    (t76372, t76373, t76394)
}
