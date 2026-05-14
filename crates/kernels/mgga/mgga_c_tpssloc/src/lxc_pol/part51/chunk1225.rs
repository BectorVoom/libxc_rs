//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1225/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1225<F: Float>(t33409: F, t6547: F, t1888: F, t31333: F, t86873: F, t1880: F, t8547: F, t87782: F, t23204: F, t33408: F, t6562: F, t33447: F, t81651: F, t82074: F, t114866: F, t6552: F, t7479: F) -> (F, F, F, F, F, F) {
    let t121296 = t6547 * t33409;
    let t121299 = t1888 * t86873 * t31333;
    let t121302 = t1880 * t87782 * t8547;
    let t121305 = t6562 * t23204 * t33408;
    let t121308 = t81651 * t82074 * t33447;
    let t121311 = t6552 * t114866 * t7479;
    (t121296, t121299, t121302, t121305, t121308, t121311)
}
