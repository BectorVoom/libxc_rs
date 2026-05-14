//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1065/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1065<F: Float>(t28155: F, t28185: F, t1378: F, t1375: F, t1843: F, t20029: F, t20044: F, t2016: F, t22646: F, t26184: F, t26345: F, t26477: F, t26988: F, t26993: F, t28051: F, t28053: F, t28108: F, t28111: F, t28118: F, t5215: F, t568: F, t6461: F, t6958: F, t7729: F, t7750: F) -> (F, F, F) {
    let t28186 = t28155 + t28185;
    let t28187 = t1378 * t28186;
    let t28190 = 0.76763589786250567036e-1 * t26184 + t26988 + t28051 * t568 + t26993 + 2.0 * t28053 * t568 + t28108 * t568 + 2.0 * t1375 * t28111 - 2.0 * t5215 * t7750 - t22646 + 0.3289868133696452873e-1 * t28118 - 2.0 * t26477 * t1843 - t6958 * t6461 + 4.0 * t5215 * t7729 - t20044 * t2016 - 2.0 * t20029 * t2016 - t1375 * t28187 + 0.82246703342411321824e-2 * t26345;
    (t28186, t28187, t28190)
}
