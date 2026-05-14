//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 969/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk969<F: Float>(t1218: F, t1232: F, t2134: F, t32425: F, t32429: F, t32433: F, t32436: F, t32441: F, t32445: F, t32448: F, t488: F, t7316: F, t7326: F, t8875: F, t466: F, t1170: F, t8891: F) -> (F, F, F) {
    let t32451 = t32425 - 0.40372756094140390856e-3 * t7316 * t8875 - 0.40372756094140390856e-3 * t2134 * t32429 + 0.40372756094140390856e-3 * t7326 * t32433 + t32436 * t488 / 1536.0 + t32441 * t1218 / 1536.0 + t32445 - t32448 * t1232 / 2304.0;
    let t32452 = t466 * t32451;
    let t32454 = t1170 * t8891;
    (t32451, t32452, t32454)
}
