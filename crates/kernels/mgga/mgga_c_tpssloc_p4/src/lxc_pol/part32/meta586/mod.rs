//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta586 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1970;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1971;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1972;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1973;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta586<F: Float>(t5979: F, t7286: F, t7285: F, t5975: F, t27820: F, t8002: F, t1238: F, t24589: F, t27808: F, t27818: F, t29795: F, t29798: F, t29804: F, t29809: F, t5055: F, t6268: F, t7283: F, t7351: F, t8088: F, t1761: F, t19232: F, t19234: F, t19249: F, t2155: F, t24587: F, t27401: F, t27406: F, t27830: F, t29667: F, t29699: F, t4945: F, t8006: F, t8015: F, t8061: F, t265: F, t504: F, t1256: F, t1763: F, t193: F, t24909: F, t27838: F, t28755: F, t336: F, t4700: F, t6270: F, t6274: F, t7398: F, t28: F, t1409: F, t2161: F, t28802: F, t52: F, t5398: F, t8097: F, t29514: F, t2165: F, t5493: F, t113: F, t1442: F, t1774: F, t28815: F, t28819: F, t28822: F, t28825: F, t28829: F, t28833: F, t28837: F, t28841: F, t28843: F, t28861: F, t28863: F, t28866: F, t29493: F, t4028: F, t510: F, t5450: F, t5457: F, t652: F, t7983: F, t7989: F, t8103: F, dens_threshold: F, rho1: F, zeta_threshold: F, t29506: F, t3: F, t1458: F, t24972: F, t27921: F, t28888: F, t28890: F, t28892: F, t28895: F, t28898: F, t28901: F, t28903: F, t5456: F, t577: F, t7423: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29812, t29813, t29816, t29817, t29822, t29825) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1970::<F>(t5979, t7286, t7285, t5975, t27820, t8002, t1238, t24589, t27808, t27818, t29795, t29798, t29804, t29809, t5055, t6268, t7283, t7351, t8088);
        let t29827 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1971::<F>(t1761, t19232, t19234, t19249, t2155, t24587, t27401, t27406, t27830, t29667, t29699, t29825, t4945, t8006, t8015, t8061, t8088);
        let t29840 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1972::<F>(t265, t504, t1256, t1763, t193, t24909, t27838, t28755, t29827, t336, t4700, t6270, t6274, t7398);
        let (t29848, t29855, t29864) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1973::<F>(t28, t1409, t2161, t28802, t29840, t52, t5398, t8097, t29514, t2165, t5493, t113, t1442, t1774, t28815, t28819, t28822, t28825, t28829, t28833, t28837, t28841, t28843, t28861, t28863, t28866, t29493, t4028, t510, t5450, t5457, t652, t7983, t7989, t8103, dens_threshold, rho1, zeta_threshold);
        let (t29865, t29866, t29884) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1974::<F>(t29506, t29864, t3, t1458, t24972, t27921, t28888, t28890, t28892, t28895, t28898, t28901, t28903, t5456, t5493, t577, t7423);
    (t29812, t29813, t29816, t29817, t29822, t29827, t29840, t29848, t29855, t29865, t29866, t29884)
}
