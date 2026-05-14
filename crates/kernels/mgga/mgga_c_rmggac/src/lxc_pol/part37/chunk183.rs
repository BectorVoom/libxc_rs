//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 183/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk183<F: Float>(t1131: F, t156: F, t155: F, t2: F, t388: F, t428: F, t180: F, t214: F, t243: F, t426: F, t194: F, t231: F, t449: F, t453: F, t452: F, t197: F) -> (F, F, F, F, F, F, F) {
    let t1132 = t156 * t1131;
    let t1133 = t155 * t1132;
    let t1134 = t388 * t2;
    let t1135 = t1134 * t428;
    let t1138 = t243 * t214 * t180;
    let t1140 = 0.24415263074675393405e-3 * t426 * t1138;
    let t1143 = t194 * t231;
    let t1152 = t449 * t453;
    let t1156 = 1.0 / t452 / t194;
    let t1157 = t197 * t1156;
    (t1133, t1135, t1140, t1143, t1152, t1156, t1157)
}
