//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 985/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk985<F: Float>(t71210: F, t74961: F, t74965: F, t14623: F, t6355: F, t14626: F, t5055: F, t2039: F, t2479: F, t270: F, t638: F, t2046: F, t2050: F, t31: F) -> (F, F, F, F, F, F, F) {
    let t77299 = F::cast_from(0.36021158228745895953e-3_f64) * t71210;
    let t77300 = F::cast_from(0.20455996240684006298e-1_f64) * t74961;
    let t77301 = F::cast_from(0.2727466165424534173e-1_f64) * t74965;
    let t77302 = t6355 * t14623;
    let t77303 = F::cast_from(0.2993560425465952141e-1_f64) * t77302;
    let t77304 = t5055 * t14626;
    let t77305 = F::cast_from(0.44903406381989282115e-1_f64) * t77304;
    let t77308 = t638 * t2039 * t2479 * t270;
    let t77309 = F::cast_from(0.15243824895787514157e-3_f64) * t77308;
    let t77312 = t2046 * t2050 * t2479 * t31;
    (t77299, t77300, t77301, t77303, t77305, t77309, t77312)
}
