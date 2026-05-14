//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 676/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk676<F: Float>(t2019: F, t6999: F, t1983: F, t1873: F, t3938: F, t671: F, t3941: F, t1401: F, t6534: F, t33: F, t63: F, t2240: F, t625: F, t67: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7014 = 0.135e2 * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = 27.0 * t3941 * t7015;
    let t7019 = 0.135e2 * t1401 * t6534;
    let t7025 = t33 * t63;
    let t7026 = t2240 * t7025;
    let t7031 = t625 * t67;
    let t7032 = t7031 * t1864;
    (t7000, t7001, t7014, t7015, t7017, t7019, t7025, t7026, t7031, t7032)
}
