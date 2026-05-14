//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 217/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk217<F: Float>(t109: F, t659: F, t95: F, t103: F, t100: F, t657: F, t92: F, t96: F, t656: F, t64: F, t654: F) -> (F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t660 = t95 * t659;
    let t662 = -t659;
    let t663 = t103 * t662;
    let t666 = 5.0 / 3.0 * t100 * t663 - 5.0 / 3.0 * t657 * t96 + 5.0 / 3.0 * t92 * t660;
    let t667 = t656 * t666;
    let t671 = piecewise3(t110, 0.0, -t654 - t64 * t667 / 8.0);
    (t660, t662, t663, t666, t667, t671)
}
