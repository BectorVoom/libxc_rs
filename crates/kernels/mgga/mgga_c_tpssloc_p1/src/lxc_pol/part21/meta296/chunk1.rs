//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1617/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1617<F: Float>(t248: F, t3101: F, t3121: F, t1020: F, t698: F, t999: F, t973: F, t2960: F, t3139: F, t1030: F, t363: F, t3068: F) -> (F, F, F, F, F, F, F) {
    let t10908 = t248 * t3101 * t3121;
    let t10909 = t1020 * t10908;
    let t10922 = t698 * t999;
    let t10923 = t973 * t10922;
    let t10927 = t2960 * t3139;
    let t10935 = t363 * t1030;
    let t10936 = t10935 * t3068;
    (t10908, t10909, t10922, t10923, t10927, t10935, t10936)
}
