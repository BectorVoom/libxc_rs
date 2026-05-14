//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 966/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk966<F: Float>(t24972: F, t7015: F, t6534: F, t7423: F, t1190: F, t8882: F, t7313: F, t8875: F, t2147: F, t7319: F, t7327: F, t7330: F, t1201: F, t8878: F, t1209: F, t483: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31942 = t24972 * t7015;
    let t31944 = t7423 * t6534;
    let t32422 = t1190 * t8882;
    let t32425 = 0.40372756094140390856e-3 * t7313 * t8875;
    let t32428 = t2147 * sigma2;
    let t32429 = t7319 * t32428;
    let t32432 = t7327 * sigma2;
    let t32433 = t32432 * t7330;
    let t32436 = t1201 * t8878;
    let t32439 = t1209 * t483;
    (t31942, t31944, t32422, t32425, t32428, t32429, t32432, t32433, t32436, t32439)
}
