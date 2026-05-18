//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 848/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk848<F: Float>(t6883: F, t8612: F, t8511: F, t9239: F, t131: F, t7025: F, t2240: F, t1862: F, t31: F, t625: F, t8301: F, t8515: F) -> (F, F, F, F, F, F, F, F) {
    let t31662 = t6883 * t8612;
    let t31663 = F::new(0.19190897446562641759e-1) * t31662;
    let t31675 = t9239 * t8511;
    let t31680 = t7025 * t131;
    let t31681 = t2240 * t31680;
    let t31682 = t1862 * t31;
    let t31687 = t8301 * t625;
    let t31688 = t2240 * t31687;
    let t31690 = F::new(5.0) / F::new(27.0) * t31688 * t8515;
    (t31663, t31675, t31680, t31681, t31682, t31687, t31688, t31690)
}
