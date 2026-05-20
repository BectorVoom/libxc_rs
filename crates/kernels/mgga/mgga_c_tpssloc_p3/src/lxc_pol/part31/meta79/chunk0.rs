//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 498/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk498<F: Float>(t1557: F, t893: F, t1541: F, t917: F, t1548: F, t1551: F, t1554: F, t926: F, t929: F, t932: F) -> (F, F, F, F) {
    let t1559 = F::new(1.0) * t893 * t1557;
    let t1561 = -t917 - F::cast_from(0.17123333333333333333e-1_f64) * t1541;
    let t1568 = F::new(0.3529725e1) * t1548 - t926 - F::new(0.516475e0) * t1541 + F::new(0.6311625e0) * t1551 - t929 - F::new(0.104195e0) * t1554;
    let t1569 = t1568 * t932;
    (t1559, t1561, t1568, t1569)
}
