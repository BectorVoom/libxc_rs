//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 857/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk857<F: Float>(t248: F, t3101: F, t5873: F, t3130: F, t376: F, t5866: F, t2970: F, t5824: F, t973: F, t5828: F, t10231: F, t5817: F, t2989: F, t5398: F, t2987: F, t5836: F) -> (F, F, F, F, F, F, F, F) {
    let t17667 = t248 * t3101 * t5873;
    let t17668 = t3130 * t17667;
    let t17712 = t376 * t5866;
    let t17763 = t2970 * t5824;
    let t17764 = t973 * t17763;
    let t17769 = t2970 * t5828;
    let t17770 = t973 * t17769;
    let t17783 = t10231 * t5817;
    let t17784 = t973 * t17783;
    let t17794 = t2989 * t5398;
    let t17800 = t2987 * t5836;
    (t17667, t17668, t17712, t17764, t17770, t17784, t17794, t17800)
}
