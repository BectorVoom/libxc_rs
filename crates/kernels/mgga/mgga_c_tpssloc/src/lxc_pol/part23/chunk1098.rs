//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1098/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1098<F: Float>(t11913: F, t52834: F, t11880: F, t15908: F, t9467: F, t9882: F, t5154: F, t9919: F, t12344: F, t5234: F, t1831: F, t40059: F, t12282: F, t12290: F, t12384: F, t1827: F, t40123: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53777 = t15908 * t9467;
    let t53779 = t15908 * t9882;
    let t53798 = t5154 * t9919;
    let t53880 = t5234 * t12344;
    let t53901 = t40059 * t1831;
    let t53945 = t5234 * t12282;
    let t54020 = t5234 * t12290;
    let t54042 = t5234 * t12384;
    let t54151 = t40123 * t1827;
    (t53592, t53613, t53777, t53779, t53798, t53880, t53901, t53945, t54020, t54042, t54151)
}
