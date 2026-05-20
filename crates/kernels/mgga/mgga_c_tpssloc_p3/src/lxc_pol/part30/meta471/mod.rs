//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1760;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1761;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1762;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta471<F: Float>(t3030: F, t344: F, t1014: F, t1011: F, t360: F, t225: F, t6733: F, t1949: F, t2966: F, t1920: F, t6680: F, t6781: F, t6805: F, t968: F, t210: F, t6795: F, t6688: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t23602 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1760::<F>(t3030, t344);
        let (t23603, t23604, t23613) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1761::<F>(t1014, t23602, t1011, t360, t225, t6733);
        let (t23617, t23619, t23626, t23629, t23631) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1762::<F>(t1949, t2966, t1920, t6680, t6781, t6805, t968, t210, t6795);
        let (t23632, t23633) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1763::<F>(t6688, t974, t23631);
    (t23602, t23603, t23604, t23613, t23617, t23619, t23626, t23629, t23631, t23632, t23633)
}
