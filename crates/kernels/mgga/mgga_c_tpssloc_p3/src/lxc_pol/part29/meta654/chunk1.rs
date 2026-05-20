//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2180/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2180<F: Float>(t12719: F, t72: F, t79: F, t1410: F, t9228: F, t2235: F, t3961: F, t3967: F, t26117: F, t6534: F, t1268: F, t86604: F) -> (F, F, F, F, F, F) {
    let t90334 = t72 * t79 * t12719;
    let t90337 = t9228 * t1410;
    let t90340 = t2235 * t3961;
    let t90343 = t2235 * t3967;
    let t90355 = F::new(4.0) * t26117 * t6534;
    let t90361 = F::new(2.0) * t1268 * t86604;
    (t90334, t90337, t90340, t90343, t90355, t90361)
}
