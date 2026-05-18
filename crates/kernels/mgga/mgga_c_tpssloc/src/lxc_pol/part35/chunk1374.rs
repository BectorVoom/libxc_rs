//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1374/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1374<F: Float>(t1888: F, t232: F, t6646: F, t67392: F, t67350: F, t82018: F, t9975: F, t22996: F, t2632: F, t67405: F, t25038: F, t25248: F, t25249: F, t5544: F) -> (F, F, F, F, F, F) {
    let t105621 = t1888 * t6646 * t67392 * t232;
    let t105629 = t1888 * t82018 * t67350 * t9975;
    let t105634 = t1888 * t22996 * t67350 * t2632;
    let t105638 = t1888 * t6646 * t67350 * t232;
    let t105642 = t1888 * t6646 * t67405 * t232;
    let t105646 = t25038 * t25248 * t25249 * t5544;
    (t105621, t105629, t105634, t105638, t105642, t105646)
}
