//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 909/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk909<F: Float>(t8660: F, t8664: F, t2573: F, t875: F, t296: F, t2576: F, t304: F, t2549: F, t872: F, t2593: F, t891: F, t2618: F, t309: F, t8772: F, t8749: F, t650: F, t969: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8871 = 0.16068111111111111111e1 * t8660;
    let t8872 = 0.46308888888888888888e0 * t8664;
    let t8887 = 1.0 / t2573 / t875;
    let t8888 = t296 * t8887;
    let t8890 = 1.0 / t2576 / t304;
    let t8899 = t872 * t2549;
    let t8906 = t891 * t2593;
    let t8912 = t891 * t2618;
    let t8915 = t309 * t8772;
    let t8922 = t309 * t8749;
    let t8927 = 0.53272592592592592592e-1 * t8660;
    let t8951 = t650 * t969;
    (t8871, t8872, t8888, t8890, t8899, t8906, t8912, t8915, t8922, t8927, t8951)
}
