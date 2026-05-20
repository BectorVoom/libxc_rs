//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2383/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2383<F: Float>(t1284: F, t17: F, t9861: F, t2225: F, t3696: F, t1285: F, t9212: F, t2516: F, t3681: F, t12126: F, t592: F, t1287: F, t9218: F) -> (F, F, F, F, F, F) {
    let t39620 = t17 * t1284 * t9861;
    let t39628 = t2225 * t3696;
    let t39636 = t9212 * t1285;
    let t39642 = t17 * t3681 * t2516;
    let t39644 = t592 * t12126;
    let t39655 = F::new(480.0) * t9218 * t1287;
    (t39620, t39628, t39636, t39642, t39644, t39655)
}
