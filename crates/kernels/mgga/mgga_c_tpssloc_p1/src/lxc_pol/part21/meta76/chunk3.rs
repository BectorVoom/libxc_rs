//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 555/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk555<F: Float>(t1541: F, t1548: F, t1551: F, t1554: F, t926: F, t929: F) -> F {
    let t1568 = F::cast_from(0.3529725e1_f64) * t1548 - t926 - F::cast_from(0.516475e0_f64) * t1541 + F::cast_from(0.6311625e0_f64) * t1551 - t929 - F::cast_from(0.104195e0_f64) * t1554;
    t1568
}
