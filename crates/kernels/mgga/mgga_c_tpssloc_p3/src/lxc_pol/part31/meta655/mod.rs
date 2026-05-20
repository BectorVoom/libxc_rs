//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta655<F: Float>(t17004: F, t6581: F, t16662: F, t1894: F, t236: F, t6591: F, t5568: F, t81956: F, t28389: F, t81963: F, t25068: F, t4257: F, t16853: F, t6621: F, t16946: F, t16951: F, t23053: F, t5619: F, t23083: F, t28356: F, t25093: F, t7496: F, t87504: F, t25115: F, t87451: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98703, t98707, t98709, t98711, t98715) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936::<F>(t17004, t6581, t16662, t1894, t236, t6591, t5568, t81956, t28389, t81963, t25068, t4257);
        let (t98717, t98719, t98721, t98723, t98725, t98728, t98731) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1937::<F>(t16853, t6621, t16946, t16951, t23053, t5619, t23083, t28356, t25093, t7496, t87504, t25115, t87451);
    (t98703, t98707, t98709, t98711, t98715, t98717, t98719, t98721, t98723, t98725, t98728, t98731)
}
