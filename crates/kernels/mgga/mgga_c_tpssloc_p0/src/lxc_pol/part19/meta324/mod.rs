//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta324<F: Float>(t32253: F, t59: F, t154: F, t541: F, t3850: F, t550: F, t12289: F, t1336: F, t835: F, t12293: F, t12364: F, t3777: F, t1354: F, t12365: F, t3853: F, t12267: F, t3789: F, t3798: F, t12297: F, t12385: F, t12300: F, t3858: F, t12402: F, t12407: F, t12409: F, t12413: F, t12429: F, t1341: F, t1343: F, t3795: F, t3803: F, t3805: F, t820: F) -> (F, F, F, F, F) {
        let (t39933, t39934, t39936, t39937, t39938, t39945, t39947) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1152::<F>(t32253, t59, t154, t541, t3850, t550, t12289, t1336, t835, t12293, t12364, t3777);
        let t39970 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153::<F>(t1354, t39947, t12365, t3853, t12267, t3789, t3798, t12297, t12385, t12300, t3858, t12402, t12407, t12409, t12413, t12429, t1341, t1343, t3795, t3803, t3805, t39936, t39938, t39945, t820);
    (t39933, t39934, t39937, t39938, t39970)
}
