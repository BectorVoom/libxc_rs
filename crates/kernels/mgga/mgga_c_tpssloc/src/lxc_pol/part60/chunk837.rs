//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 837/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk837<F: Float>(t114891: F, t2047: F, t212: F, t23171: F, t6554: F, t23228: F, t8547: F, t193: F, t201: F, t8565: F, t10143: F, t531: F, t8639: F, t22716: F, t8622: F, t2085: F, t22642: F, t6890: F) -> (F, F, F, F, F, F, F, F) {
    let t114892 = 0.26044789391763585244e-1 * t114891;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = 0.82246703342411321824e-2 * t114932;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = 0.82246703342411321824e-2 * t114943;
    let t115009 = t193 * t201 * t8565;
    let t115027 = t8565 * t10143;
    let t115262 = t531 * t8639;
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1 * t115305;
    let t115330 = t22642 * t212 * t2085 * t6890;
    (t114892, t114933, t114944, t115009, t115027, t115262, t115306, t115330)
}
