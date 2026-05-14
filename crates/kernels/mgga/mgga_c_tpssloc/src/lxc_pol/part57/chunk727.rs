//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 727/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk727<F: Float>(t26167: F, t7687: F, t1983: F, t191: F, t192: F, t6295: F, t2020: F, t20085: F, t2019: F, t1390: F, t6330: F, t22595: F, t1799: F, t1845: F, t8643: F, t22574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28817 = t26167 * t7687;
    let t28819 = 6.0 * t1983 * t28817;
    let t28821 = t6295 * t191 * t192;
    let t28822 = t28821 * t2020;
    let t28823 = t2019 * t20085;
    let t28825 = 2.0 * t1983 * t28823;
    let t28826 = t1390 * t6330;
    let t28827 = t22595 * t28826;
    let t28829 = 6.0 * t1983 * t28827;
    let t28830 = t1799 * t1845;
    let t28831 = t8643 * t28830;
    let t28833 = 6.0 * t22574 * t28831;
    (t28817, t28819, t28821, t28822, t28823, t28825, t28826, t28827, t28829, t28830, t28831, t28833)
}
