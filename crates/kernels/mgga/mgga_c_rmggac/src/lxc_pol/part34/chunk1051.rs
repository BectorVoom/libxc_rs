//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1051/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1051<F: Float>(t76379: F, t76381: F, t69213: F, t69234: F, t69241: F, t69250: F, t69261: F, t75300: F, t75304: F, t69274: F, t75308: F, t69201: F, t69206: F, t69245: F, t71269: F, t71270: F, t71271: F, t71272: F) -> (F, F, F) {
    let t78119 = F::new(0.2727466165424534173e-1) * t76379;
    let t78120 = F::new(0.54549323308490683461e-1) * t76381;
    let t78122 = F::new(0.77145928569998943516e-3) * t69213;
    let t78123 = F::new(0.16566831523319392755e-1) * t69234;
    let t78124 = F::new(0.27611385872198987926e-1) * t69241;
    let t78125 = F::new(0.72732431077987577944e-1) * t69250;
    let t78126 = F::new(0.10286123809333192469e-2) * t69261;
    let t78127 = F::new(0.77145928569998943515e-3) * t75300;
    let t78128 = F::new(0.10286123809333192468e-2) * t75304;
    let t78129 = F::new(0.26609426004141796809e-1) * t69274;
    let t78130 = F::new(0.19914231157590872009e-2) * t75308;
    let t78131 = t69201 - F::new(0.33868944250243438615e-2) * t69206 - t78122 - t78123 + t78124 + t69245 - t78125 + t78126 + t71269 + t71270 - t71271 + t71272 - t78127 + t78128 - t78129 + t78130;
    (t78119, t78120, t78131)
}
