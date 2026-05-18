//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 936/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk936<F: Float>(t12571: F, t31680: F, t115876: F, t33564: F, t31688: F, t33572: F, t45844: F, t8511: F, t33115: F, t31687: F, t8515: F, t33409: F, t6547: F) -> (F, F, F, F, F, F, F) {
    let t121058 = t12571 * t31680;
    let t121064 = t115876 * t33564;
    let t121066 = t31688 * t33572;
    let t121094 = t45844 * t8511;
    let t121121 = t31688 * t33115;
    let t121124 = t12571 * t31687 * t8515;
    let t121296 = t6547 * t33409;
    (t121058, t121064, t121066, t121094, t121121, t121124, t121296)
}
