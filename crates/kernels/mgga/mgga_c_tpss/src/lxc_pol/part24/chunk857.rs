//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 857/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk857<F: Float>(t6113: F, t626: F, t1338: F, t1753: F, t1364: F, t30: F, t1713: F, t1369: F, t5547: F, t1381: F, t5552: F, t1385: F, t5559: F, t5546: F, t5556: F) -> (F, F, F, F, F) {
    let t6115 = 2.0 * t626 * t6113;
    let t6117 = t1753 * t1338;
    let t6120 = t30 * t1364;
    let t6121 = t1713 * t6120;
    let t6124 = t5547 * t1369;
    let t6126 = t5552 * t1381;
    let t6128 = t5559 * t1385;
    let t6130 = -t5546 - t6124 / 48.0 - t6126 / 1536.0 - t5556 - t6128 / 384.0;
    (t6115, t6117, t6120, t6121, t6130)
}
