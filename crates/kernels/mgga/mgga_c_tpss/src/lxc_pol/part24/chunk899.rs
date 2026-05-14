//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 899/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk899<F: Float>(t785: F, t8286: F, t2158: F, t339: F, t789: F, t2387: F, t72: F, t240: F, t769: F, t790: F, t2162: F, t750: F, t810: F, t73: F, t2157: F, t806: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8287 = t8286 * t785;
    let t8292 = t339 * t2158 * t789;
    let t8305 = t2387 * t72;
    let t8306 = t8305 * t240;
    let t8313 = t339 * t769 * t790;
    let t8325 = t2162 * t750;
    let t8346 = t810 * t810;
    let t8347 = 1.0 / t8346;
    let t8348 = t73 * t8347;
    let t8361 = t2157 * t806;
    (t8287, t8292, t8305, t8306, t8313, t8325, t8346, t8347, t8348, t8361)
}
