//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2250/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2250<F: Float>(t25068: F, t4257: F, t16853: F, t6621: F, t16946: F, t16951: F, t23053: F, t5619: F, t23083: F, t28356: F, t25093: F, t7496: F, t87504: F) -> (F, F, F, F, F, F, F) {
    let t98715 = t25068 * t4257;
    let t98717 = t6621 * t16853;
    let t98719 = t6621 * t16946;
    let t98721 = t6621 * t16951;
    let t98723 = t23053 * t5619;
    let t98725 = t23083 * t28356;
    let t98728 = t87504 * t7496 * t25093;
    (t98715, t98717, t98719, t98721, t98723, t98725, t98728)
}
