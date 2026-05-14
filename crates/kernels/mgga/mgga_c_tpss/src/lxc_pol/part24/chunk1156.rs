//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1156/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1156<F: Float>(t1322: F, t1753: F, t19304: F, t19307: F, t19310: F, t19312: F, t19315: F, t19318: F, t19322: F, t19324: F, t19326: F, t19329: F, t3491: F, t3538: F, t3542: F, t5514: F, t5692: F, t626: F) -> (F,) {
    let t19334 = -t1322 * t5692 - t1753 * t3491 - 2.0 * t19315 * t626 - 2.0 * t19318 * t626 - 2.0 * t3538 * t5514 - 2.0 * t3542 * t5514 - t19304 - t19307 - t19310 - t19312 - t19322 - t19324 - t19326 - t19329;
    (t19334,)
}
