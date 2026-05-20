//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2127/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2127<F: Float>(t19451: F, t6535: F, t22574: F, t28830: F, t31035: F, t1390: F, t19631: F, t1983: F, t6878: F, t25989: F, t91655: F, t1845: F, t5356: F) -> (F, F, F, F, F) {
    let t96815 = F::new(2.0) * t19451 * t6535;
    let t96818 = F::new(6.0) * t22574 * t31035 * t28830;
    let t96824 = t1390 * t19631;
    let t96827 = F::new(3.0) * t1983 * t6878 * t96824;
    let t96829 = F::new(6.0) * t91655 * t25989;
    let t96830 = t1845 * t5356;
    (t96815, t96818, t96827, t96829, t96830)
}
