//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 485/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk485<F: Float>(t221: F, t446: F, t6108: F, t1468: F, t1494: F, t1875: F, t4559: F, t489: F, t490: F, t6067: F, t1228: F, t1900: F) -> (F, F, F, F, F, F) {
    let t6110 = t221 * t6108 * t446;
    let t6113 = t1468 * t1494;
    let t6114 = t221 * t6113;
    let t6117 = t4559 * t1875;
    let t6120 = t489 * t490 * t6067;
    let t6123 = t1228 * t1900;
    (t6110, t6113, t6114, t6117, t6120, t6123)
}
