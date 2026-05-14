//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 902/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk902<F: Float>(t15887: F, t290: F, t289: F, t77283: F, t77286: F, t77287: F, t77288: F, t77293: F, t77297: F, t77299: F, t77300: F, t77301: F, t77303: F, t77305: F, t77309: F, t77313: F, t77317: F, t77321: F, t77322: F) -> (F,) {
    let t80183 = t290 * t15887;
    let t80186 = -t77283 + t77286 - t77287 + t77288 + t77293 - t77297 + t77299 - t77300 + t77301 + t77303 - t77305 - t77309 + t77313 - t77317 + t77321 - 0.2363e1 * t289 * t80183 - t77322;
    (t80186,)
}
