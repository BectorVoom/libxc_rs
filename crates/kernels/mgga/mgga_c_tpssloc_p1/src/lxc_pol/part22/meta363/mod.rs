//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1607;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1608;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1609;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta363<F: Float>(t10817: F, t5695: F, t2787: F, t5727: F, t10296: F, t10556: F, t10675: F, t10676: F, t13551: F, t13552: F, t13563: F, t13567: F, t17173: F, t17180: F, t17185: F, t13598: F, t13650: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t13642: F, t13645: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t913: F, t893: F, t10655: F, t5730: F, t5737: F, t923: F, t5775: F, t950: F, t1581: F, t4471: F, t10740: F, t14263: F, t14266: F, t14337: F, t1569: F, t2856: F, t2905: F, t2930: F, t4411: F, t4434: F, t4454: F, t4476: F, t5743: F, t5759: F, t933: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17377, t17379, t17398) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1607::<F>(t10817, t5695, t2787, t5727, t10296, t10556, t10675, t10676, t13551, t13552, t13563, t13567, t17173, t17180, t17185);
        let t17422 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1608::<F>(t13598, t13650, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t13645, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17398);
        let (t17423, t17425, t17427, t17428, t17443, t17446, t17449) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1609::<F>(t17422, t913, t893, t10655, t5730, t5737, t923, t5775, t950, t1581, t4471, t10740, t14263, t14266, t14337, t1569, t17377, t17379, t2856, t2905, t2930, t4411, t4434, t4454, t4476, t5743, t5759, t933);
    (t17377, t17379, t17422, t17423, t17425, t17427, t17428, t17443, t17446, t17449)
}
