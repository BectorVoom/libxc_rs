//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1759;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1760;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta469<F: Float>(t461: F, t52: F, t1009: F, t7324: F, t1210: F, t7330: F, t3502: F, t3504: F, t3500: F, sigma2: F, t7337: F, t1202: F, t7344: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1759::<F>(t461, t52, t1009, t7324, t1210, t7330, t3502, t3504, t3500, sigma2);
        let (t24732, t24733) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1760::<F>(t3504, t7337, t3500);
        let t24736 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1761::<F>(t1202, t7344);
    (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729, t24732, t24733, t24736)
}
