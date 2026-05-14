//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1249/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1249<F: Float>(t68: F, t7594: F, t2009: F, t588: F, t43: F, t7737: F, t789: F, t582: F, t7690: F, t31455: F, t5486: F, t18341: F, t31464: F, t2: F, t823: F, t1288: F, t2436: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t61877 = t68 * t7594;
    let t61964 = t588 * t2009;
    let t61969 = t43 * t7737;
    let t61976 = 1232.0 / 27.0 * t789;
    let t62019 = t7690 * t582;
    let t62027 = t31455 * t5486;
    let t62030 = t7690 * t18341;
    let t62033 = t31464 * t5486;
    let t63783 = t823 * t2;
    let t63840 = t2436 * t1288;
    (t61877, t61964, t61969, t61976, t62019, t62027, t62030, t62033, t63783, t63840)
}
