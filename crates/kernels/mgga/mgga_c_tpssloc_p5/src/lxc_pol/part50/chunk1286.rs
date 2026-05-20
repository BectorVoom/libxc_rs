//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1286/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1286<F: Float>(t2022: F, t7774: F, t1851: F, t8509: F, t33196: F, t576: F, t55353: F, t8319: F, t16524: F, t31280: F, t23880: F, t26550: F) -> (F, F, F, F, F, F) {
    let t120774 = t2022 * t7774;
    let t120780 = t1851 * t8509;
    let t120783 = t576 * t33196;
    let t120786 = F::new(27.0) * t55353 * t8319;
    let t120788 = F::new(54.0) * t16524 * t31280;
    let t120789 = t23880 * t26550;
    (t120774, t120780, t120783, t120786, t120788, t120789)
}
