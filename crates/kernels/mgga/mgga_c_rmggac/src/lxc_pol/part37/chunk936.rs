//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 936/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk936<F: Float>(t76945: F, t74259: F, t74262: F, t74267: F, t74269: F, t74272: F, t74275: F, t74278: F, t74287: F, t74295: F, t74316: F, t74324: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76946 = F::new(0.15243824895787514157e-3) * t76945;
    let t76947 = F::new(0.30487649791575028312e-3) * t74259;
    let t76948 = F::new(0.72042316457491791901e-3) * t74262;
    let t76949 = F::new(0.16263363996404810741e-4) * t74267;
    let t76950 = F::new(0.38430329123504567781e-4) * t74269;
    let t76951 = F::new(0.16263363996404810741e-4) * t74272;
    let t76952 = F::new(0.72042316457491791901e-3) * t74275;
    let t76955 = F::new(0.72042316457491791901e-3) * t74278;
    let t76957 = F::new(0.2553875993597870364e-4) * t74287;
    let t76959 = F::new(0.1702583995731913576e-4) * t74295;
    let t76965 = F::new(0.85129199786595678799e-5) * t74316;
    let t76968 = F::new(0.15961724959986689775e-4) * t74324;
    (t76946, t76947, t76948, t76949, t76950, t76951, t76952, t76955, t76957, t76959, t76965, t76968)
}
