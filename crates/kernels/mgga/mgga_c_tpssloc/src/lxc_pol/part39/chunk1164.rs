//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1164/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1164<F: Float>(t109: F, t30056: F, t8184: F, t2358: F, t8180: F, t2349: F, t99: F, t2350: F, t2354: F, t29903: F, t30048: F, t30049: F, t30051: F, t30053: F, t8128: F, t8137: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t30057 = t8184 * t30056;
    let t30060 = t8180 * t2358;
    let t30063 = t99 * t2349;
    let t30064 = t30063 * t2350;
    let t30067 = t8184 * t2354;
    let t30071 = piecewise3(t110, 0.0, -t30048 - 4.0 / 3.0 * t30049 + 10.0 / 9.0 * t30051 - 3.0 / 4.0 * t29903 * t30053 + 5.0 / 6.0 * t8128 * t30057 + t8128 * t30060 / 4.0 - 5.0 / 36.0 * t8137 * t30064 - 5.0 / 24.0 * t8137 * t30067);
    (t30057, t30060, t30063, t30064, t30067, t30071)
}
