//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 948/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk948<F: Float>(t23204: F, t33408: F, t6562: F, t33447: F, t81651: F, t82074: F, t2717: F, t7841: F, t33448: F, t81591: F, t8547: F, t86893: F) -> (F, F, F, F, F) {
    let t121305 = t6562 * t23204 * t33408;
    let t121308 = t81651 * t82074 * t33447;
    let t121349 = t2717 * t7841;
    let t121371 = t81591 * t33448;
    let t121399 = t6562 * t86893 * t8547;
    (t121305, t121308, t121349, t121371, t121399)
}
