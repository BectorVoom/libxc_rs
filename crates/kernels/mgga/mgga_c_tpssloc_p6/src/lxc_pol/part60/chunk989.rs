//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 989/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk989<F: Float>(t1799: F, t7752: F, t28030: F, t8327: F, t32677: F, t7458: F, t20162: F, t8326: F, t28893: F, t33194: F, t16524: F, t33193: F) -> (F, F, F, F, F, F, F) {
    let t127553 = t1799 * t7752;
    let t127560 = F::new(2.0) * t28030 * t8327;
    let t127562 = F::new(4.0) * t7458 * t32677;
    let t127601 = F::new(0.135e2) * t20162 * t8326;
    let t127603 = F::new(27.0) * t28893 * t8326;
    let t127606 = F::new(54.0) * t33194;
    let t127608 = F::new(54.0) * t16524 * t33193;
    (t127553, t127560, t127562, t127601, t127603, t127606, t127608)
}
