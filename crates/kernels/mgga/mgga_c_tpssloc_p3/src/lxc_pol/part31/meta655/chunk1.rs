//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1937/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1937<F: Float>(t16853: F, t6621: F, t16946: F, t16951: F, t23053: F, t5619: F, t23083: F, t28356: F, t25093: F, t7496: F, t87504: F, t25115: F, t87451: F) -> (F, F, F, F, F, F, F) {
    let t98717 = t6621 * t16853;
    let t98719 = t6621 * t16946;
    let t98721 = t6621 * t16951;
    let t98723 = t23053 * t5619;
    let t98725 = t23083 * t28356;
    let t98728 = t87504 * t7496 * t25093;
    let t98731 = t87451 * t7496 * t25115;
    (t98717, t98719, t98721, t98723, t98725, t98728, t98731)
}
