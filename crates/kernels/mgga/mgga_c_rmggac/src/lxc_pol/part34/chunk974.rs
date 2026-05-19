//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 974/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk974<F: Float>(t74708: F, t2010: F, t2012: F, t9639: F, t15496: F, t2019: F, t2020: F, t68796: F, t74718: F, t74722: F, t74725: F, t68892: F, t71042: F, t71043: F, t71046: F, t71054: F, t74713: F, t74716: F, t74730: F, t74734: F, t74739: F) -> F {
    let t77164 = F::cast_from(0.7883687741805615234e-5_f64) * t74708;
    let t77166 = t2010 * t2012 * t9639;
    let t77167 = F::cast_from(0.36021158228745895953e-3_f64) * t77166;
    let t77169 = t2019 * t2020 * t15496;
    let t77170 = F::cast_from(0.15243824895787514157e-3_f64) * t77169;
    let t77171 = F::cast_from(0.1921128438866447784e-2_f64) * t68796;
    let t77172 = F::cast_from(0.638468998399467591e-4_f64) * t74718;
    let t77173 = F::cast_from(0.72042316457491791901e-3_f64) * t74722;
    let t77174 = F::cast_from(0.38430329123504567781e-4_f64) * t74725;
    let t77179 = t77164 - t77167 - t77170 - t71042 + t71043 + t77171 + t71046 + t74713 + t74716 - t71054 + t77172 + t77173 - t77174 - F::cast_from(0.17451485956252114154e-4_f64) * t74730 + F::cast_from(0.34902971912504228308e-4_f64) * t74734 + F::cast_from(0.29085809927086856923e-4_f64) * t68892 - F::cast_from(0.69805943825008456616e-4_f64) * t74739;
    t77179
}
