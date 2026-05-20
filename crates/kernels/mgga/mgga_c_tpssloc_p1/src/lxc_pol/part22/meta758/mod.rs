//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta758 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2547;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta758<F: Float>(t1113: F, t136: F, t71189: F, t71201: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t1102: F, t5999: F, t14801: F, t14804: F, t45192: F, t48140: F, t68513: F, t50822: F, t44938: F, t43777: F, t43859: F, t43895: F, t50919: F, t50948: F, t71203: F, t71206: F, t43816: F, t51040: F, t51051: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t64074: F, t64076: F, t64087: F, t64089: F, t71343: F, t71396: F, t71428: F, t71440: F, t71467: F, t51402: F, t6024: F, t21961: F, t44162: F, t21810: F, t3259: F, t50834: F, t51137: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F, t71333: F, t71335: F, t71337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71486, t71489, t71494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544::<F>(t1113, t136, t71189, t71201, t71191, t71195, t71199, t71468, t71470, t71472, t71474, t71477, t71480, t71483);
        let (t71499, t71501, t71505, t71508, t71511, t71515) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2545::<F>(t1102, t5999, t14801, t14804, t45192, t48140, t68513, t50822, t44938, t43777, t43859, t43895, t50919, t50948, t71203, t71206);
        let t71527 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2546::<F>(t43816, t51040, t51051, t63361, t63382, t63384, t63398, t63400, t64074, t64076, t64087, t64089);
        let (t71530, t71543, t71545) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2547::<F>(t71343, t71396, t71428, t71440, t71467, t71494, t71515, t71527, t51402, t6024, t21961, t44162);
        let (t71547, t71558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2548::<F>(t21810, t3259, t50834, t51137, t63291, t63306, t63308, t63841, t63843, t63845, t71333, t71335, t71337);
    (t71486, t71489, t71499, t71501, t71505, t71508, t71511, t71530, t71543, t71545, t71547, t71558)
}
