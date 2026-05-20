//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1641;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta449<F: Float>(t5: F, t24006: F, t112: F, t1268: F, t12734: F, t12739: F, t2039: F, t2314: F, t2363: F, t23917: F, t23938: F, t23941: F, t5113: F, t671: F, t7042: F, t7056: F, t9348: F, t6999: F, t7217: F, t22754: F, t22757: F, t22762: F, t22766: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22784: F, t22786: F, t22789: F, t22795: F, t22798: F, t22800: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22805: F, t22809: F, t22830: F, t22834: F, t22837: F, t22840: F, t22848: F, t22850: F, t22856: F, t22860: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24007, t24008, t24026) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1640::<F>(t5, t24006, t112, t1268, t12734, t12739, t2039, t2314, t2363, t23917, t23938, t23941, t5113, t671, t7042, t7056, t9348);
        let (t24028, t24046) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1641::<F>(t6999, t7217, t22754, t22757, t22762, t22766, t22768, t22771, t22774, t22777, t22780, t22784, t22786, t22789, t22795, t22798, t22800);
        let (t24049, t24050, t24058, t24060, t24061, t24062) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1642::<F>(t22819, t22825, t22858, t22863, t22867, t22805, t22809, t22830, t22834, t22837, t22840, t22848, t22850, t22856, t22860);
    (t24007, t24008, t24026, t24028, t24046, t24049, t24050, t24058, t24060, t24061, t24062)
}
