//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 934/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk934<F: Float>(t131: F, t2108: F, t39063: F, t8662: F, t31863: F, t9239: F, t22573: F, t8689: F, t63: F, t8308: F, t113875: F, t625: F, t79: F) -> (F, F, F, F, F, F, F, F) {
    let t116065 = t2108 * t131;
    let t116075 = t39063 * t8662;
    let t116106 = t9239 * t31863;
    let t116114 = t8662 * t131;
    let t116115 = t9239 * t116114;
    let t116135 = t8689 * t22573;
    let t117447 = t8308 * t63;
    let t117451 = t113875 * t63;
    let t117477 = t116065 * t117447;
    let t117480 = t79 * t625;
    (t116075, t116106, t116115, t116135, t117447, t117451, t117477, t117480)
}
