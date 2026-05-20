//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk657;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk658;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk659;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk660;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk661;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta102<F: Float>(t2303: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t609: F, t629: F, t642: F, t66: F, t80: F, t5: F, t2233: F, t2235: F, t2240: F, t2241: F, t605: F, t645: F, t86: F, t112: F, t111: F, t649: F, t671: F, t89: F, t1266: F, t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F, t666: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2304, t2307) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk657::<F>(t2303, t72, t2245, t2252, t2255, t2284, t609, t629, t642, t66, t80);
        let (t2311, t2312) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk658::<F>(t5, t2233, t2235, t2240, t2241, t2307, t605, t645, t86, t112);
        let t2314 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk659::<F>(t111, t649);
        let t2319 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk660::<F>(t671);
        let (t2320, t2323, t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk661::<F>(t2319, t89, t1266, t671, t107, t2281, t626, t667, t106, t655);
        let t2332 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk662::<F>(t666);
    (t2304, t2307, t2311, t2312, t2314, t2319, t2320, t2323, t2327, t2328, t2331, t2332)
}
