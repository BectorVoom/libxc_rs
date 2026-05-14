//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 815/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk815<F: Float>(t11552: F, t221: F, t456: F, t1176: F, t3242: F, t10471: F, t11715: F, t11712: F, t11721: F, t6739: F, t3502: F, t3508: F, t1209: F, t475: F, t3639: F, t500: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11832 = t221 * t11552;
    let t11834 = 5.0 / 1296.0 * t456 * t11832;
    let t11848 = t1176 * t3242;
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11883 = t6739 * t11721;
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11915 = t6739 * t475;
    let t11947 = 1.0 / t3639 / t500;
    (t11832, t11834, t11848, t11880, t11881, t11883, t11887, t11888, t11889, t11913, t11914, t11915, t11947)
}
