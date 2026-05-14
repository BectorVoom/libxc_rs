//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 938/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk938<F: Float>(t1094: F, t3263: F, t3266: F, t1118: F, t11191: F, t3313: F, t1157: F, t3395: F, t3403: F, t1155: F, t1138: F, t3351: F, t1136: F, t3359: F, t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11424 = t1094 * t3263;
    let t11426 = 6.0 * t11424 * t3266;
    let t11427 = t11191 * t1118;
    let t11429 = 6.0 * t3313 * t11427;
    let t11430 = t1157 * t3395;
    let t11433 = t3395 * t3403;
    let t11434 = t11433 * t1155;
    let t11437 = t1138 * t3351;
    let t11441 = t3351 * t3359 * t1136;
    let t11444 = 0.53272592592592592592e-1 * t11135;
    let t11455 = -t11444 + 0.2283111111111111111e-1 * t11137 + 0.11415555555555555555e-1 * t11139 - 0.34246666666666666665e-1 * t11141 - 0.17123333333333333333e-1 * t11143 + 0.19025925925925925925e-1 * t11150 - 0.68493333333333333331e-1 * t11156 - 0.34246666666666666665e-1 * t11161 + 0.10274e0 * t11165 + 0.10274e0 * t11170 + 0.17123333333333333333e-1 * t11174;
    (t11424, t11426, t11427, t11429, t11430, t11433, t11434, t11437, t11441, t11455)
}
