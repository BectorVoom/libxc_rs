//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta199 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk937;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk938;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk939;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk940;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta199<F: Float>(t1340: F, t5234: F, t1358: F, t1815: F, t1362: F, t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t120: F, t1824: F, t1351: F, t3792: F, t1827: F, t3799: F, t1315: F, t1354: F, t1369: F, t3733: F, t3762: F, t3763: F, t3778: F, t5220: F, t5223: F, t5227: F, t5231: F, t559: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5235, t5238, t5240, t5245, t5246, t5247, t5248) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk937::<F>(t1340, t5234, t1358, t1815, t1362, t242, t3788, t1336, t557, t67, t246);
        let t5249 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk938::<F>(t120, t1824);
        let t5250 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk939::<F>(t1351, t3792);
        let t5252 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk940::<F>(t5248, t5249, t5250);
        let (t5255, t5257) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk941::<F>(t1827, t3799, t1315, t1354, t1369, t3733, t3762, t3763, t3778, t5220, t5223, t5227, t5231, t5235, t5238, t5240, t5246, t5252, t559);
    (t5235, t5238, t5240, t5245, t5246, t5247, t5248, t5249, t5250, t5252, t5255, t5257)
}
