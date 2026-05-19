//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1073/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1073<F: Float>(t69600: F, t71369: F, t71373: F, t71376: F, t75115: F, t75134: F, t75143: F, t77463: F, t77464: F, t77465: F, t77468: F, t77471: F, t77474: F, t77476: F, t77477: F, t77480: F, t77481: F) -> F {
    let t80223 = t75115 + t77463 - t77464 + t77465 - t77468 - t77471 - t77474 - t71369 - t77476 + t71373 - t71376 - t77477 + t69600 - F::cast_from(0.17519306092901367186e-5_f64) * t75134 - t77480 - t77481 + t75143;
    t80223
}
