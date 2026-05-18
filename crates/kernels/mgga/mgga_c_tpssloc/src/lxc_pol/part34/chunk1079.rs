//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1079/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1079<F: Float>(t29274: F, t29285: F, t539: F, t1807: F, t7918: F, t2085: F, t6361: F, t12021: F, t2091: F, t6439: F, t1842: F, t7936: F) -> (F, F, F, F, F, F) {
    let t29286 = t29274 + t29285;
    let t29287 = t539 * t29286;
    let t29290 = t1807 * t7918;
    let t29293 = t6361 * t2085;
    let t29299 = t12021 * t2091 * t6439;
    let t29310 = t7936 * t1842;
    (t29286, t29287, t29290, t29293, t29299, t29310)
}
