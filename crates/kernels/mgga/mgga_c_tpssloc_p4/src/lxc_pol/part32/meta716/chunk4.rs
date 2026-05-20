//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2263/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2263<F: Float>(t98644: F, t98688: F, t98713: F, t98740: F, t98795: F, t98816: F, t98846: F, t98873: F, t25038: F, t25248: F, t776: F, t98422: F) -> (F, F) {
    let t98876 = t98644 + t98688 + t98713 + t98740 + t98795 + t98816 + t98846 + t98873;
    let t98881 = t25038 * t25248 * t98422 * t776;
    (t98876, t98881)
}
