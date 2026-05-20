//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2041;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta586<F: Float>(t22989: F, t81591: F, t22690: F, t23153: F, t23171: F, t6561: F, t80741: F, t6643: F, t23025: F, t23030: F, t23012: F, t6653: F, t22641: F, t2588: F, t225: F, t814: F, t6648: F, t23021: F, t6547: F, t23155: F, t23168: F, t22893: F, t23158: F, t23164: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81592, t81595, t81597, t81599, t81600, t81602) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2041::<F>(t22989, t81591, t22690, t23153, t23171, t6561, t80741, t6643, t23025, t23030, t23012, t6653);
        let (t81612, t81613, t81615, t81617, t81623, t81630) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2042::<F>(t22641, t2588, t225, t814, t6648, t23021, t6547, t23155, t23168, t22893, t23158, t23164);
    (t81592, t81595, t81597, t81599, t81600, t81602, t81612, t81613, t81615, t81617, t81623, t81630)
}
