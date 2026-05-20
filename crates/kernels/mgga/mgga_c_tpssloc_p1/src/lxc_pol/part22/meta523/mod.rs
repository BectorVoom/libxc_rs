//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1990;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta523<F: Float>(t1755: F, t22368: F, t22364: F, t3625: F, t22327: F, t493: F, t22243: F, t491: F, t1246: F, t1751: F, t6218: F, t11881: F, t11888: F, t11914: F, t1244: F, t15027: F, t15245: F, t1729: F, t1756: F, t1758: F, t19201: F, t22114: F, t22341: F, t22349: F, t22355: F, t22358: F, t22361: F, t22365: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F, t6253: F, t6257: F, t6261: F, t6263: F, t6265: F, t1241: F, t22113: F, t1238: F, t1761: F, t19232: F, t19234: F, t19249: F, t22004: F, t22008: F, t22328: F, t22334: F, t22337: F, t4945: F, t498: F, t5055: F, t6244: F, t6268: F, t1256: F, t1763: F, t19267: F, t193: F, t21956: F, t21958: F, t21960: F, t21963: F, t21990: F, t22224: F, t22226: F, t22231: F, t22235: F, t22239: F, t22241: F, t336: F, t4700: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22369, t22372, t22375, t22386, t22387, t22389, t22390, t22393) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1989::<F>(t1755, t22368, t22364, t3625, t22327, t493, t22243, t491, t1246, t1751, t6218, t11881, t11888, t11914, t1244, t15027, t15245, t1729, t1756, t1758, t19201, t22114, t22341, t22349, t22355, t22358, t22361, t22365, t3610, t3624, t470, t494, t5064, t6168, t6253, t6257, t6261, t6263, t6265);
        let (t22394, t22398, t22408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1990::<F>(t1241, t22393, t22113, t491, t1238, t1761, t19232, t19234, t19249, t22004, t22008, t22328, t22334, t22337, t4945, t498, t5055, t6244, t6268);
        let t22412 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1991::<F>(t1256, t1763, t19267, t193, t21956, t21958, t21960, t21963, t21990, t22224, t22226, t22231, t22235, t22239, t22241, t22408, t336, t4700);
    (t22369, t22372, t22375, t22386, t22387, t22389, t22390, t22393, t22394, t22398, t22408, t22412)
}
