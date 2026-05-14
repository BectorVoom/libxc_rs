//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1286/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1286<F: Float>(t1689: F, t69026: F, t21180: F, t5522: F, t1165: F, t68888: F, t1688: F, t42710: F, t50656: F, t13565: F, t5531: F, t69023: F, t13133: F, t6112: F, t13554: F, t19596: F, t3493: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t69028 = 4.0 * t69026 * t1689;
    let t69030 = 4.0 * t21180 * t5522;
    let t69032 = 2.0 * t1165 * t68888;
    let t69051 = 2.0 * t42710 * t1688;
    let t69053 = 2.0 * t50656 * t1688;
    let t69055 = 2.0 * t13565 * t5531;
    let t69057 = 4.0 * t69023 * t1688;
    let t69059 = 4.0 * t69026 * t1688;
    let t69062 = 4.0 * t21180 * t5531;
    let t69064 = 4.0 * t13133 * t6112;
    let t69066 = 4.0 * t13554 * t6112;
    let t69068 = 4.0 * t3493 * t19596;
    (t69028, t69030, t69032, t69051, t69053, t69055, t69057, t69059, t69062, t69064, t69066, t69068)
}
