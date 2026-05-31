//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1470/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1470<F: Float>(t10647: F, t291: F, t2784: F, t892: F, t914: F, t2787: F, t2837: F, t2841: F, t888: F) -> (F, F, F, F, F) {
    let t10649 = F::cast_from(0.621814e-1_f64) * t10647 * t291;
    let t10650 = t2784 * t892;
    let t10652 = F::cast_from(3.0_f64) * t10650 * t914;
    let t10654 = F::cast_from(3.0_f64) * t2787 * t2837;
    let t10655 = t888 * t2841;
    (t10649, t10650, t10652, t10654, t10655)
}
