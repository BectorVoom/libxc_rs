//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1762;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1763;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta470<F: Float>(t483: F, t3068: F, t1244: F, sigma2: F, t2132: F, t24683: F, t225: F, t460: F, t479: F, t3523: F, t7345: F, t3572: F, t7339: F, t24574: F, t7368: F, t2148: F, t3427: F, t2121: F, t24594: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24739, t24740, t24741) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1762::<F>(t483, t3068, t1244, sigma2);
        let (t24744, t24745, t24746) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1763::<F>(t2132, t24683, t225, t460, t479);
        let (t24747, t24752, t24754, t24760, t24771, t24773, t24776) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1764::<F>(t24744, t24746, t3523, t7345, t3572, t7339, t24574, t7368, t2148, t3427, t2121, t225, t24594);
    (t24739, t24740, t24741, t24745, t24746, t24747, t24752, t24754, t24760, t24771, t24773, t24776)
}
