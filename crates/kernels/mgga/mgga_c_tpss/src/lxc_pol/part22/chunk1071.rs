//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1071/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1071<F: Float>(t1324: F, t2084: F, t7613: F, t2083: F, t97: F, t1989: F, t633: F, t1990: F, t3514: F, t100: F, t555: F, t22: F, t3518: F, t1329: F, t2092: F, t7629: F) -> (F, F, F, F, F, F, F) {
    let t13178 = t7613 * t1324 * t2084;
    let t13181 = t97 * t2083;
    let t13182 = t1989 * t633;
    let t13185 = t3514 * t1990;
    let t13188 = t100 * t555;
    let t13191 = t3518 * t22;
    let t13199 = t7629 * t1329 * t2092;
    (t13178, t13181, t13182, t13185, t13188, t13191, t13199)
}
