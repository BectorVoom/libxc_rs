//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 830/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk830<F: Float>(t45844: F, t8511: F, t31688: F, t33115: F, t12571: F, t31687: F, t8515: F, t33409: F, t6547: F, t23204: F, t33408: F, t6562: F, t33447: F, t81651: F, t82074: F, t2717: F, t7841: F) -> (F, F, F, F, F, F, F) {
    let t121094 = t45844 * t8511;
    let t121121 = t31688 * t33115;
    let t121124 = t12571 * t31687 * t8515;
    let t121296 = t6547 * t33409;
    let t121305 = t6562 * t23204 * t33408;
    let t121308 = t81651 * t82074 * t33447;
    let t121349 = t2717 * t7841;
    (t121094, t121121, t121124, t121296, t121305, t121308, t121349)
}
