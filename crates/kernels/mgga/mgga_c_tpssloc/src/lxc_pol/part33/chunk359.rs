//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 359/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk359<F: Float>(t1625: F, t349: F, t1615: F, t381: F, t1060: F, t383: F, t1058: F, t1610: F, t353: F, t384: F) -> (F, F, F, F, F) {
    let t1626 = t349 * t1625;
    let t1629 = t381 * t1615;
    let t1630 = t1629 * t1060;
    let t1632 = t383 * t1625;
    let t1634 = t1058 * t1630 + t1610 * t384 + t1632 * t353;
    (t1626, t1629, t1630, t1632, t1634)
}
