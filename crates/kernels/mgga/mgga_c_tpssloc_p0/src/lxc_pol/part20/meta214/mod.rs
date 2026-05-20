//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta214 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1259;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1260;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1261;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1262;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1263;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1264;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1265;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta214<F: Float>(t242: F, t3788: F, t1336: F, t557: F, t67: F, t246: F, t120: F, t1824: F, t1351: F, t3792: F, t1827: F, t3799: F, t1315: F, t1354: F, t1369: F, t3733: F, t3762: F, t3763: F, t3778: F, t5220: F, t5223: F, t5227: F, t5231: F, t5235: F, t5238: F, t5240: F, t559: F, t3805: F, t3807: F, t2408: F, t2417: F, t2423: F, t3686: F, t3688: F, t3690: F, t3695: F, t3813: F, t5153: F, t5156: F, t5159: F, t5164: F, t5167: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5245, t5246) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1259::<F>(t242, t3788, t1336);
        let (t5247, t5248) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1260::<F>(t557, t67, t246);
        let t5249 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1261::<F>(t120, t1824);
        let t5250 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1262::<F>(t1351, t3792);
        let t5252 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1263::<F>(t5248, t5249, t5250);
        let t5257 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1264::<F>(t1827, t3799, t1315, t1354, t1369, t3733, t3762, t3763, t3778, t5220, t5223, t5227, t5231, t5235, t5238, t5240, t5246, t5252, t559);
        let t5259 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1265::<F>(t3805, t3807, t5249);
        let t5262 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1266::<F>(t2408, t2417, t2423, t3686, t3688, t3690, t3695, t3813, t5153, t5156, t5159, t5164, t5167);
    (t5245, t5246, t5247, t5248, t5249, t5250, t5252, t5257, t5259, t5262)
}
