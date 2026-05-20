//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1931/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1931<F: Float>(t16914: F, t23146: F, t16903: F, t5593: F, t81749: F, t16845: F, t25084: F, t16893: F, t17017: F, t16841: F, t87368: F, t25083: F, t4166: F, t4184: F) -> (F, F, F, F, F, F, F, F) {
    let t98614 = t23146 * t16914;
    let t98616 = t23146 * t16903;
    let t98618 = t81749 * t5593;
    let t98620 = t25084 * t16845;
    let t98622 = t25084 * t16893;
    let t98624 = t23146 * t17017;
    let t98626 = t87368 * t16841;
    let t98629 = t4166 * t25083 * t4184;
    (t98614, t98616, t98618, t98620, t98622, t98624, t98626, t98629)
}
