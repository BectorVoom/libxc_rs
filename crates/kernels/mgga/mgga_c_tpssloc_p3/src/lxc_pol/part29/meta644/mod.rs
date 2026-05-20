//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta644 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2122;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2123;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2124;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2125;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2126;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta644<F: Float>(t2631: F, t47285: F, t6605: F, t9972: F, t12971: F, t1894: F, t236: F, t6591: F, t23046: F, t4184: F, t812: F, t836: F, t13080: F, t23146: F, t242: F, t81816: F, t13265: F, t13333: F, t25084: F, t13076: F, t13084: F, t87329: F, t87331: F, t87333: F, t87336: F, t87339: F, t87342: F, t87343: F, t87345: F, t87348: F, t87351: F, t25083: F, t2617: F, t13244: F, t25064: F, t81788: F, t13193: F, t6621: F, t13198: F, t23097: F, t232: F, t46565: F, t815: F, t46644: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F, t6604: F, t81968: F, t13184: F, t841: F, t23083: F, t25123: F, t81912: F, t81887: F, t81889: F, t81899: F, t81903: F, t81909: F, t13191: F, t25119: F, t1878: F, t81982: F, t221: F, t25120: F, t81962: F, t13196: F, t13204: F, t6581: F) -> (F, F, F, F, F, F, F) {
        let (t87355, t87359, t87363) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2122::<F>(t2631, t47285, t6605, t9972, t12971, t1894, t236, t6591, t23046, t4184, t812, t836);
        let t87377 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2123::<F>(t87363, t13080, t23146, t242, t812, t81816, t13265, t13333, t25084, t13076, t13084, t87329, t87331, t87333, t87336, t87339, t87342, t87343, t87345, t87348, t87351, t87355, t87359);
        let (t87379, t87381, t87387, t87389, t87391, t87395) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2124::<F>(t25083, t2617, t4184, t13244, t25084, t25064, t81788, t13193, t6621, t13198, t23097, t232, t46565, t815);
        let (t87399, t87402, t87403, t87405, t87407) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2125::<F>(t23097, t232, t46644, t815, t25135, t838, t2693, t7503, t25132, t81882, t6604, t81968);
        let t87415 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2126::<F>(t13184, t841, t87407, t23083, t25123, t81912, t81887, t81889, t81899, t81903, t81909, t87379, t87381, t87387, t87389, t87391, t87395, t87399, t87402, t87403, t87405);
        let (t87418, t87422, t87426, t87428, t87430) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2127::<F>(t13191, t25119, t841, t1878, t81982, t13184, t221, t25120, t6604, t81962, t13196, t13204, t6581);
    (t87377, t87415, t87418, t87422, t87426, t87428, t87430)
}
