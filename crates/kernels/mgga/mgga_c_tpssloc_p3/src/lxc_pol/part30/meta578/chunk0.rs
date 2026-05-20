//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1954/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1954<F: Float>(t1983: F, t28827: F, t1799: F, t1845: F, t8643: F, t22574: F, t1390: F, t6347: F, t6878: F, t7685: F, t7688: F, t7754: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28829 = F::new(6.0) * t1983 * t28827;
    let t28830 = t1799 * t1845;
    let t28831 = t8643 * t28830;
    let t28833 = F::new(6.0) * t22574 * t28831;
    let t28834 = t1390 * t6347;
    let t28835 = t6878 * t28834;
    let t28837 = F::new(3.0) * t1983 * t28835;
    let t28841 = F::new(6.0) * t7685 * t7688;
    let t28843 = F::new(2.0) * t7685 * t7754;
    (t28829, t28830, t28831, t28833, t28834, t28835, t28837, t28841, t28843)
}
