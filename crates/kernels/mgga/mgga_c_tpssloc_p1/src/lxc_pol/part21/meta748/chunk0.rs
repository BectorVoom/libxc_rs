//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2620/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2620<F: Float>(t15908: F, t9882: F, t118: F, t2375: F, t5151: F, t16169: F, t2663: F, t15892: F, t2371: F, t5154: F, t9919: F, t5173: F, t591: F) -> (F, F, F, F, F, F) {
    let t53779 = t15908 * t9882;
    let t53782 = t5151 * t118 * t2375;
    let t53787 = t16169 * t2663;
    let t53796 = t15892 * t2371;
    let t53798 = t5154 * t9919;
    let t53825 = F::new(16.0) * t5173 * t591;
    (t53779, t53782, t53787, t53796, t53798, t53825)
}
