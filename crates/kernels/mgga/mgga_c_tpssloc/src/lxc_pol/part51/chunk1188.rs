//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1188/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1188<F: Float>(t2240: F, t31680: F, t1862: F, t31: F, t607: F, t8308: F, t625: F, t8301: F) -> (F, F, F, F, F) {
    let t31681 = t2240 * t31680;
    let t31682 = t1862 * t31;
    let t31683 = t31682 * t607;
    let t31684 = t8308 * t31683;
    let t31687 = t8301 * t625;
    let t31688 = t2240 * t31687;
    (t31681, t31682, t31684, t31687, t31688)
}
