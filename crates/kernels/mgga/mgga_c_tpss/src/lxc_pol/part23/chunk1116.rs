//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1116/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1116<F: Float>(t1259: F, t13108: F, t10171: F, t1256: F, t1266: F, t13033: F, t13035: F, t13047: F, t13051: F, t13055: F, t1657: F, t3360: F, t3367: F, t3385: F, t4490: F, t4494: F, t4517: F, t538: F) -> (F, F) {
    let t13109 = t1259 * t13108;
    let t13111 = -t10171 * t1657 - 6.0 * t1256 * t13047 + 4.0 * t1256 * t13051 + 2.0 * t1256 * t13055 - t1256 * t13109 - 2.0 * t1266 * t13035 + t13033 * t538 + 4.0 * t3360 * t4494 - 2.0 * t3360 * t4517 + 2.0 * t3367 * t4490 - t3385 * t4490;
    (t13109, t13111)
}
