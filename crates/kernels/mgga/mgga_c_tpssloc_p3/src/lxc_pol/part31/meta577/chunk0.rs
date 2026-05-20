//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1813/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1813<F: Float>(t22751: F, t26186: F, t26190: F, t26356: F, t6914: F, t1799: F, t3886: F, t80663: F, t80671: F, t1887: F, t80827: F, t26334: F) -> (F, F, F, F, F, F, F, F) {
    let t90468 = t22751 * t26186;
    let t90470 = t22751 * t26190;
    let t90472 = t6914 * t26356;
    let t90488 = t3886 * t1799;
    let t90493 = F::cast_from(0.12793931631041761173e0_f64) * t80663;
    let t90496 = F::cast_from(0.10417915756705434098e0_f64) * t80671;
    let t90497 = t80827 * t1887;
    let t90498 = t90497 * t26334;
    (t90468, t90470, t90472, t90488, t90493, t90496, t90497, t90498)
}
