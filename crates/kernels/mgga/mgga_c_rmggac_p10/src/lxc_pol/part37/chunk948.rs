//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 948/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk948<F: Float>(t74705: F, t74708: F, t2010: F, t2012: F, t9639: F, t15496: F, t2019: F, t2020: F, t68796: F, t74718: F, t74722: F, t74725: F) -> (F, F, F, F, F, F, F, F) {
    let t77162 = F::cast_from(0.5255791827870410156e-5_f64) * t74705;
    let t77164 = F::cast_from(0.7883687741805615234e-5_f64) * t74708;
    let t77166 = t2010 * t2012 * t9639;
    let t77167 = F::cast_from(0.36021158228745895953e-3_f64) * t77166;
    let t77169 = t2019 * t2020 * t15496;
    let t77170 = F::cast_from(0.15243824895787514157e-3_f64) * t77169;
    let t77171 = F::cast_from(0.1921128438866447784e-2_f64) * t68796;
    let t77172 = F::cast_from(0.638468998399467591e-4_f64) * t74718;
    let t77173 = F::cast_from(0.72042316457491791901e-3_f64) * t74722;
    let t77174 = F::cast_from(0.38430329123504567781e-4_f64) * t74725;
    (t77162, t77164, t77167, t77170, t77171, t77172, t77173, t77174)
}
