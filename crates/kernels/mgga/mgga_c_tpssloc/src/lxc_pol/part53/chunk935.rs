//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 935/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk935<F: Float>(t111: F, t7945: F, t1307: F, t1842: F, t1527: F, t776: F, t2098: F, t671: F, t7786: F, t23109: F, t23110: F, t232: F, t59: F, t828: F) -> (F, F, F, F, F, F) {
    let t94170 = t7945 * t111;
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    let t100993 = t2098 * t671;
    let t102344 = t7786 * t671;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    (t94170, t97721, t98960, t100993, t102344, t112778)
}
