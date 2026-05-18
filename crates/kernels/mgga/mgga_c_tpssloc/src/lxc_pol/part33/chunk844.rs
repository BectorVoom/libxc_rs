//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 844/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk844<F: Float>(t10471: F, t3502: F, t11712: F, t3508: F, t6739: F, t1209: F, t475: F, t3639: F, t500: F, t1287: F, t2223: F, t1291: F, t9874: F) -> (F, F, F, F, F, F, F) {
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11913 = t10471 * t1209;
    let t11914 = t11712 * t11913;
    let t11915 = t6739 * t475;
    let t11947 = F::new(1.0) / t3639 / t500;
    let t11981 = t2223 * t1287;
    let t11982 = F::new(96.0) * t11981;
    let t11984 = F::new(0.56968947174242584612e-3) * t1291 * t9874;
    (t11888, t11889, t11914, t11915, t11947, t11982, t11984)
}
