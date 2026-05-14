//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 655/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk655<F: Float>(t1043: F, t2863: F, t2862: F, t392: F, t395: F, t1027: F, t2834: F, t2836: F, t2843: F, t2848: F, t2852: F, t1025: F, t1032: F, t2509: F, t275: F, t400: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2864 = t2863 * t1043;
    let t2866 = 2.0 * t2862 * t2864;
    let t2868 = 1.0 / t395 / t392;
    let t2869 = t1027 * t1027;
    let t2870 = t2868 * t2869;
    let t2872 = 4.0 / 9.0 * t2834;
    let t2877 = t2872 - 2.0 / 9.0 * t2836 - 2.0 / 9.0 * t2843 + 2.0 / 3.0 * t2848 + t2852 / 3.0;
    let t2878 = t1025 * t2877;
    let t2880 = 0.39862222222222222223e0 * t2834;
    let t2885 = 1.0/f64::sqrt(t392);
    let t2886 = t2885 * t2869;
    let t2888 = t1032 * t2877;
    let t2891 = t275 * t2509 * t400;
    (t2864, t2866, t2868, t2869, t2870, t2872, t2877, t2878, t2880, t2885, t2886, t2888, t2891)
}
