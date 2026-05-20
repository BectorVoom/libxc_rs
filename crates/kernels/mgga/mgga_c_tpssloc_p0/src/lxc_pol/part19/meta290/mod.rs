//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1061;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1063;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta290<F: Float>(t12364: F, t1336: F, t1354: F, t120: F, t3791: F, t1307: F, t3792: F, t3805: F, t1328: F, t210: F, t3719: F, t12178: F, t1343: F, t820: F, t3788: F, t835: F, t3795: F, t3799: F, t3853: F, t12353: F, t12356: F, t12358: F, t12361: F, t1341: F, t1363: F, t3733: F, t3778: F, t3858: F, t5246: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12365, t12366, t12368) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1061::<F>(t12364, t1336, t1354, t120, t3791);
        let (t12369, t12371, t12375, t12379, t12384) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062::<F>(t1307, t3792, t12368, t3805, t1328, t210, t3719, t12178, t1343, t820, t3788, t835);
        let (t12385, t12390) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1063::<F>(t12384, t1336, t3795, t3799, t3853, t12353, t12356, t12358, t12361, t12366, t12371, t12375, t12379, t1341, t1363, t3733, t3778, t3858, t5246);
    (t12365, t12368, t12369, t12371, t12375, t12379, t12384, t12385, t12390)
}
