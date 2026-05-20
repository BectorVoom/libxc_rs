//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta429 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1755;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1756;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1757;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1758;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta429<F: Float>(t1238: F, t15820: F, t1761: F, t18287: F, t19121: F, t19209: F, t19211: F, t19214: F, t19220: F, t19226: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5060: F, t6268: F, t225: F, t6151: F, t6153: F, t6239: F, t1720: F, t5052: F, t1751: F, t4940: F, t18571: F, t491: F, t1252: F, t14972: F, t14980: F, t15797: F, t5089: F, t6244: F, t1256: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F, t18909: F, t18913: F, t193: F, t336: F, t4700: F, t5091: F, t5095: F, t3640: F, t6270: F, t11947: F, t6274: F, t1254: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18837: F, t18839: F, t18917: F, t18920: F, t18922: F, t18924: F, t18928: F, t18930: F, t18932: F, t18936: F, t18938: F, t28: F, t265: F, t504: F, t17133: F, t1081: F, t1260: F, t1409: F, t1649: F, t16558: F, t17141: F, t1768: F, t18196: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t5398: F, t5669: F, t5966: F, t607: F, t6279: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t19231 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1755::<F>(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
        let (t19232, t19234, t19249, t19253, t19256, t19259, t19261) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1756::<F>(t225, t6151, t6153, t6239, t1720, t5052, t1751, t4940, t18571, t491, t1252, t14972, t14980, t15797, t1761, t3487, t3593, t4945, t498, t5055, t5089, t6244);
        let (t19262, t19266) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1757::<F>(t19231, t19261, t1256, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679, t18909, t18913, t193, t336, t4700, t5091, t5095);
        let (t19267, t19270, t19274) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1758::<F>(t3640, t6270, t11947, t6274, t1254, t18682, t18685, t18688, t18690, t18692, t18694, t18696, t18837, t18839, t18917, t18920, t18922, t18924, t18928, t18930, t18932, t18936, t18938, t4700);
        let (t19276, t19288) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1759::<F>(t28, t265, t504, t17133, t19266, t19274, t1081, t1260, t1409, t1649, t16558, t17141, t1768, t18196, t3966, t4324, t506, t5099, t52, t5398, t5669, t5966, t607, t6279, t873, dens_threshold, rho1, zeta_threshold);
    (t19232, t19234, t19249, t19253, t19256, t19259, t19262, t19267, t19270, t19276, t19288)
}
