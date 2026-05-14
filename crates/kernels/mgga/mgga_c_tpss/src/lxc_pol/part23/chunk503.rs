//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 503/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk503<F: Float>(t259: F, t479: F, t1881: F, t1884: F, t1887: F, t473: F, t1153: F, t1741: F, t198: F, t330: F) -> (F, F) {
    let t480 = t259 < t479;
    let t1889 = t1881 * t473 - t1884 * t1887;
    let t1893 = piecewise3(t480, t198 * t330 * t1889 * t1153, t1741);
    (t1889, t1893)
}
