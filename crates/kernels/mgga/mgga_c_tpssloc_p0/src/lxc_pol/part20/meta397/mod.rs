//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1790;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta397<F: Float>(t10599: F, t1547: F, t2799: F, t13615: F, t894: F, t1553: F, t2403: F, t4392: F, t699: F, t13611: F, t908: F, t136: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t10300: F, t10556: F, t10558: F, t10560: F, t10562: F, t10675: F, t10676: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13551: F, t13552: F, t13557: F, t13561: F, t13563: F, t13592: F, t13616: F, t13624: F, t13626: F) -> (F, F, F, F, F, F, F, F) {
        let (t13637, t13638, t13640, t13642, t13644, t13645, t13646, t13647) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1790::<F>(t10599, t1547, t2799, t13615, t894, t1553, t2403, t4392, t699, t13611, t908, t136);
        let t13654 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1791::<F>(t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13645, t13647, t10300, t10556, t10558, t10560, t10562, t10675, t10676, t13530, t13534, t13539, t13544, t13548, t13551, t13552, t13557, t13561, t13563, t13592, t13616, t13624, t13626);
    (t13637, t13638, t13640, t13642, t13644, t13646, t13647, t13654)
}
