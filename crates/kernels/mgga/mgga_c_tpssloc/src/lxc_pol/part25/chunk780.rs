//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 780/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk780<F: Float>(t10889: F, t3128: F, t3033: F, t248: F, t3101: F, t3121: F, t1020: F, t2250: F, t607: F, t4583: F, t4582: F, t4588: F, t698: F, t999: F, t973: F, t2960: F, t3139: F) -> (F, F, F, F, F, F, F) {
    let t10903 = t3128 * t10889;
    let t10904 = t3033 * t10903;
    let t10908 = t248 * t3101 * t3121;
    let t10909 = t1020 * t10908;
    let t10913 = t607 * t2250;
    let t10914 = t4583 * t10913;
    let t10915 = t4582 * t10914;
    let t10918 = t4588 * t10913;
    let t10919 = t4582 * t10918;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    (t10904, t10909, t10913, t10915, t10919, t10923, t10927)
}
