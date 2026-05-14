//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 871/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk871<F: Float>(t33447: F, t81651: F, t82074: F, t114866: F, t6552: F, t7479: F, t25341: F, t31366: F, t1880: F, t26679: F, t6553: F, t6571: F, t22986: F, t23270: F, t31332: F, t98960: F) -> (F, F, F, F, F) {
    let t121308 = t81651 * t82074 * t33447;
    let t121311 = t6552 * t114866 * t7479;
    let t121314 = t6552 * t31366 * t25341;
    let t121318 = t1880 * t6553 * t6571 * t26679;
    let t121326 = t22986 * t23270 * t31332 * t98960;
    (t121308, t121311, t121314, t121318, t121326)
}
