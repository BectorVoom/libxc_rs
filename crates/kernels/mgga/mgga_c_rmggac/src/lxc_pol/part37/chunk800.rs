//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 800/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk800<F: Float>(t74240: F, t70892: F, t74247: F, t74249: F, t74253: F, t74256: F, t15492: F, t2019: F, t2020: F, t74259: F, t74262: F, t74267: F, t74269: F, t74272: F, t74275: F, t74278: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76937 = 0.38430329123504567781e-4 * t74240;
    let t76939 = 0.15243824895787514157e-3 * t70892;
    let t76940 = 0.85129199786595678799e-5 * t74247;
    let t76941 = 0.85129199786595678799e-5 * t74249;
    let t76942 = 0.85129199786595678799e-5 * t74253;
    let t76943 = 0.72042316457491791901e-3 * t74256;
    let t76945 = t2019 * t2020 * t15492;
    let t76946 = 0.15243824895787514157e-3 * t76945;
    let t76947 = 0.30487649791575028312e-3 * t74259;
    let t76948 = 0.72042316457491791901e-3 * t74262;
    let t76949 = 0.16263363996404810741e-4 * t74267;
    let t76950 = 0.38430329123504567781e-4 * t74269;
    let t76951 = 0.16263363996404810741e-4 * t74272;
    let t76952 = 0.72042316457491791901e-3 * t74275;
    let t76955 = 0.72042316457491791901e-3 * t74278;
    (t76937, t76939, t76940, t76941, t76942, t76943, t76946, t76947, t76948, t76949, t76950, t76951, t76952, t76955)
}
