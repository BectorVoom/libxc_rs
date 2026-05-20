//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1766/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1766<F: Float>(t225: F, t814: F, t6648: F, t81612: F, t22715: F, t6551: F, t6640: F, t117: F, t4179: F, t6559: F, t229: F, t268: F) -> (F, F, F, F, F, F) {
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81632 = t22715 * t6551;
    let t81633 = t81632 * t6640;
    let t81640 = t6559 * t4179 * t117;
    let t81651 = t6559 * t229 * t268;
    (t81613, t81615, t81632, t81633, t81640, t81651)
}
