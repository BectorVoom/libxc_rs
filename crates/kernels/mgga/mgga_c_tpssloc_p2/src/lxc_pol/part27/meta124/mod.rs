//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk742;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk743;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk744;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk745;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta124<F: Float>(t2289: F, t2244: F, t882: F, t123: F, t2250: F, t883: F) -> (F, F, F, F, F, F, F) {
        let t2775 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk742::<F>(t2289);
        let t2776 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk743::<F>(t2244, t2775);
        let (t2777, t2778) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk744::<F>(t2776, t882, t123);
        let t2780 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk745::<F>(t2250, t883);
        let (t2781, t2782) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk746::<F>(t2780, t882, t123);
    (t2775, t2776, t2777, t2778, t2780, t2781, t2782)
}
