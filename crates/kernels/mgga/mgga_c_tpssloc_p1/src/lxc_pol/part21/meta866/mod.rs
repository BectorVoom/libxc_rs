//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta866 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta866<F: Float>(t65278: F, t65279: F, t65281: F, t65282: F, t65285: F, t65286: F, t65297: F, t65327: F, t11881: F, t11907: F, t1235: F, t1244: F, t1246: F, t14986: F, t15000: F, t15009: F, t15027: F, t15239: F, t1755: F, t18940: F, t19128: F, t19160: F, t19179: F, t3610: F, t3612: F, t3613: F, t3624: F, t3626: F, t491: F, t5064: F, t5079: F, t6260: F, t65221: F, t65254: F, t65262: F, t65265: F, t3590: F, t6224: F, t11877: F, t11904: F, t14989: F, t15004: F, t15032: F, t15248: F, t19123: F, t19139: F, t19189: F, t19201: F, t19204: F, t3617: F, t3625: F, t5011: F, t5052: F, t5080: F, t5084: F, t52435: F, t53565: F, t6261: F, t1215: F, t19120: F, t19131: F, t19145: F, t19146: F, t19154: F, t19165: F, t19176: F, t3493: F, t3507: F, t3621: F, t44753: F, t44754: F, t45329: F, t5068: F, t5069: F, t52485: F, t6238: F, t6252: F, t6257: F, t1213: F, t18941: F, t248: F, t3570: F, t15730: F, t5019: F, t1216: F, t3966: F, t1227: F, t1230: F, t15495: F, t15498: F, t15708: F, t15710: F, t15740: F, t1737: F, t1748: F, t19051: F, t3527: F, t3531: F, t3577: F, t3578: F, t3585: F, t44929: F, t44932: F, t4728: F, t5014: F, t5030: F, t53406: F, t53507: F, t5971: F, t6227: F, t6232: F, t63357: F, t63363: F) -> (F, F, F, F, F, F, F) {
        let (t65330, t65343) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3160::<F>(t65278, t65279, t65281, t65282, t65285, t65286, t65297, t65327, t11881, t11907, t1235, t1244, t1246, t14986, t15000, t15009, t15027, t15239, t1755, t18940, t19128, t19160, t19179, t3610, t3612, t3613, t3624, t3626, t491, t5064, t5079, t6260, t65221, t65254, t65262, t65265);
        let (t65347, t65374) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3161::<F>(t3590, t6224, t11877, t11904, t11907, t1244, t1246, t14989, t15004, t15027, t15032, t15248, t19123, t19139, t19189, t19201, t19204, t3617, t3624, t3625, t5011, t5052, t5064, t5079, t5080, t5084, t52435, t53565, t6261);
        let t65408 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3162::<F>(t11877, t11881, t11904, t11907, t1215, t1244, t1246, t19120, t19128, t19131, t19145, t19146, t19154, t19165, t19176, t19189, t19201, t3493, t3507, t3610, t3621, t44753, t44754, t45329, t5068, t5069, t52485, t6238, t6252, t6257);
        let (t65452, t65463) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3163::<F>(t1213, t18941, t248, t3570, t15730, t5019, t1216, t3966, t1227, t1230, t15495, t15498, t15708, t15710, t15740, t1737, t1748, t19051, t3527, t3531, t3577, t3578, t3585, t44929, t44932, t4728, t5014, t5030, t53406, t53507, t5971, t6227, t6232, t63357, t63363);
    (t65330, t65343, t65347, t65374, t65408, t65452, t65463)
}
