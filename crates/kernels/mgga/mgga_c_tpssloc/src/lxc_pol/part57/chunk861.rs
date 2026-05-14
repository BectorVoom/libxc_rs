//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 861/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk861<F: Float>(t1799: F, t7752: F, t28030: F, t8327: F, t32677: F, t7458: F, t20162: F, t8326: F, t28893: F, t33194: F, t16524: F, t33193: F, t3941: F, t5493: F, t1458: F, t1851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t127553 = t1799 * t7752;
    let t127560 = 2.0 * t28030 * t8327;
    let t127562 = 4.0 * t7458 * t32677;
    let t127601 = 0.135e2 * t20162 * t8326;
    let t127603 = 27.0 * t28893 * t8326;
    let t127606 = 54.0 * t33194;
    let t127608 = 54.0 * t16524 * t33193;
    let t127627 = 27.0 * t3941 * t8326 * t5493;
    let t127630 = t1851 * t1458;
    (t127553, t127560, t127562, t127601, t127603, t127606, t127608, t127627, t127630)
}
