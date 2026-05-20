//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1088/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1088<F: Float>(t225: F, t6401: F, t6402: F, t3843: F, t6330: F, t1347: F, t6347: F, t1819: F, t1821: F, t546: F, t548: F) -> (F, F, F, F) {
    let t6404 = (t6401 + t6402) * t225;
    let t6408 = t3843 * t6330;
    let t6411 = t1347 * t6347;
    let t6414 = F::new(6.0) * t1819 * t1821 - F::new(12.0) * t546 * t6408 + F::new(3.0) * t546 * t6411 - t548 * t6404;
    (t6404, t6408, t6411, t6414)
}
