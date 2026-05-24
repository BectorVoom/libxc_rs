//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 764/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk764<F: Float>(t1338: F, t2010: F, t35709: F, t7352: F, t31: F, t34790: F, t7349: F, t302: F, t7350: F, t7353: F, t35214: F, t7351: F) -> (F, F, F, F) {
    let t35712 = t2010 * t35709 * t7352 * t1338;
    let t35713 = F::cast_from(0.91462949374725084942e-3_f64) * t35712;
    let t35716 = t7349 * t35709 * t34790 * t31;
    let t35717 = F::cast_from(0.13010691197123848594e-3_f64) * t35716;
    let t35718 = t7350 * t302;
    let t35720 = t7349 * t35718 * t7353;
    let t35724 = t7349 * t7351 * t35214 * t31;
    (t35713, t35717, t35720, t35724)
}
