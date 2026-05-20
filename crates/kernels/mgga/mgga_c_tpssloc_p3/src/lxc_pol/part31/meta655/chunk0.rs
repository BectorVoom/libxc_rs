//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1936/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936<F: Float>(t17004: F, t6581: F, t16662: F, t1894: F, t236: F, t6591: F, t5568: F, t81956: F, t28389: F, t81963: F, t25068: F, t4257: F) -> (F, F, F, F, F) {
    let t98703 = t6581 * t17004;
    let t98707 = t6591 * t1894 * t236 * t16662;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    let t98715 = t25068 * t4257;
    (t98703, t98707, t98709, t98711, t98715)
}
