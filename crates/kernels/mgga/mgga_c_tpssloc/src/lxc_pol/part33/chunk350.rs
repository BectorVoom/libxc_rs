//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 350/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk350<F: Float>(t1541: F, t1548: F, t1551: F, t1554: F, t926: F, t929: F, t932: F, t936: F) -> (F, F, F) {
    let t1568 = F::new(0.3529725e1) * t1548 - t926 - F::new(0.516475e0) * t1541 + F::new(0.6311625e0) * t1551 - t929 - F::new(0.104195e0) * t1554;
    let t1569 = t1568 * t932;
    let t1573 = -t936 - F::new(0.92708333333333333333e-2) * t1541;
    (t1568, t1569, t1573)
}
