//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1558;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1559;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta430<F: Float>(t1995: F, t9223: F, t213: F, t1999: F, t1372: F, t552: F, t117: F, t547: F, t67: F, t6559: F, t225: F, t794: F, t6969: F, t3787: F, t6604: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22865, t22867, t22881, t22891, t22892) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1558::<F>(t1995, t9223, t213, t1999, t1372, t552, t117, t547, t67, t6559);
        let t22893 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1559::<F>(t225, t794);
        let (t22894, t22896, t22897) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1560::<F>(t22893, t6969, t22892, t3787, t6604);
    (t22865, t22867, t22881, t22891, t22892, t22893, t22894, t22896, t22897)
}
