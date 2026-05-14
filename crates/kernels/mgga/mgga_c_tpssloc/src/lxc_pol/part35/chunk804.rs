//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 804/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk804<F: Float>(t9798: F, t9860: F, t157: F, t153: F, t181: F, t686: F, t781: F, t756: F, t2371: F, t677: F, t2374: F, t2535: F, t2528: F, t2509: F, t745: F, t9843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9861 = t9798 + t9860;
    let t9862 = t157 * t9861;
    let t9863 = t153 * t9862;
    let t9874 = t686 * t781 * t181;
    let t9876 = 0.56968947174242584612e-3 * t756 * t9874;
    let t9882 = t677 * t2371;
    let t9884 = 0.32530743900905219526e-1 * t2374 * t9882;
    let t9885 = t677 * t2535;
    let t9887 = 0.16265371950452609763e-1 * t2374 * t9885;
    let t9888 = t677 * t2528;
    let t9890 = 0.48159733137676571078e0 * t2374 * t9888;
    let t9892 = t2509 * t745 * t9843;
    (t9861, t9863, t9874, t9876, t9882, t9884, t9885, t9887, t9888, t9890, t9892)
}
