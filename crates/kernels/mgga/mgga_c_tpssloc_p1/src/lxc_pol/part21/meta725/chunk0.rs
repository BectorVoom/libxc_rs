//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2580/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2580<F: Float>(t3400: F, t4832: F, t11282: F, t1687: F, t1682: F, t3357: F, t11310: F, t1694: F, t3401: F, t11420: F, t1098: F, t14956: F) -> (F, F, F, F, F, F, F) {
    let t51371 = t4832 * t3400;
    let t51376 = t1687 * t11282;
    let t51382 = t3357 * t1682;
    let t51385 = t11310 * t1694;
    let t51389 = t3401 * t1694;
    let t51392 = t11420 * t1682;
    let t51397 = t14956 * t1098;
    (t51371, t51376, t51382, t51385, t51389, t51392, t51397)
}
