//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1163/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1163<F: Float>(t64: F, t9365: F, t2199: F, t3929: F, t1266: F, t8189: F, t2196: F, t2281: F, t29895: F, t8181: F, t29900: F, t8185: F, t2332: F, t8180: F, t662: F, t666: F) -> (F, F, F, F, F, F, F, F) {
    let t29903 = t64 * t9365;
    let t30035 = t2199 * t3929;
    let t30038 = t1266 * t8189;
    let t30048 = 11.0 / 9.0 * t2281 * t2196;
    let t30049 = t29895 * t8181;
    let t30051 = t29900 * t8185;
    let t30053 = t8180 * t2332;
    let t30056 = t666 * t662;
    (t29903, t30035, t30038, t30048, t30049, t30051, t30053, t30056)
}
