//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 909/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk909<F: Float>(t2165: F, t8292: F, t2387: F, t72: F, t240: F, t2116: F, t226: F, t339: F, t769: F, t790: F, t2179: F, t2133: F, t2162: F, t2364: F, t219: F, t2399: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8293 = t8292 * t2165;
    let t8305 = t2387 * t72;
    let t8306 = t8305 * t240;
    let t8307 = t226 * t2116;
    let t8313 = t339 * t769 * t790;
    let t8314 = t8313 * t2179;
    let t8320 = t226 * t2133;
    let t8330 = t2162 * t2364;
    let t8339 = t2399 * t219;
    (t8293, t8305, t8306, t8307, t8313, t8314, t8320, t8330, t8339)
}
