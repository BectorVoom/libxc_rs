//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 644/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk644<F: Float>(t407: F, t2863: F, t2911: F, t2834: F, t2836: F, t2843: F, t2848: F, t2852: F, t1049: F, t1053: F, t1052: F, t417: F, t412: F, t1061: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2912 = t407 * t407;
    let t2913 = 1.0 / t2912;
    let t2914 = t2863 * t2913;
    let t2916 = 0.16081979498692535067e2 * t2911 * t2914;
    let t2917 = 0.22831111111111111111e-1 * t2834;
    let t2922 = t2917 - 0.11415555555555555555e-1 * t2836 - 0.11415555555555555555e-1 * t2843 + 0.34246666666666666666e-1 * t2848 + 0.17123333333333333333e-1 * t2852;
    let t2925 = t1049 * t1053;
    let t2928 = t1052 * t417;
    let t2929 = 1.0 / t2928;
    let t2930 = t412 * t2929;
    let t2931 = t1061 * t1061;
    (t2912, t2913, t2914, t2916, t2917, t2922, t2925, t2929, t2930, t2931)
}
