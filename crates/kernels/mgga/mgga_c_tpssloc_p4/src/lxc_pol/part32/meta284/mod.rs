//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1286;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1287;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1288;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta284<F: Float>(t28: F, t265: F, t504: F, t1256: F, t1763: F, t193: F, t336: F, t4700: F, t7398: F, t7642: F, t8090: F, t1409: F, t2161: F, t52: F, t7663: F, dens_threshold: F, rho1: F, zeta_threshold: F, t7997: F, t1458: F, t7266: F, t7675: F, t7678: F, t7680: F, t7983: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t2114: F, t2165: F, t2167: F, t510: F, t574: F, t652: F, t7457: F, t7460: F, t7463: F, t7470: F, t7686: F, t7690: F, t7755: F, t7757: F, t7989: F, t3: F, t577: F, t7423: F, t7768: F, t7771: F, t7773: F, t2018: F, t3701: F) -> (F, F, F, F, F, F, F) {
        let (t8097, t8102) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1286::<F>(t28, t265, t504, t1256, t1763, t193, t336, t4700, t7398, t7642, t8090, t1409, t2161, t52, t7663, dens_threshold, rho1, zeta_threshold);
        let t8103 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1287::<F>(t7997, t8102);
        let (t8107, t8110) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1288::<F>(t1458, t7266, t7675, t7678, t7680, t7983, t113, t1442, t1459, t1774, t1849, t2114, t2165, t2167, t510, t574, t652, t7457, t7460, t7463, t7470, t7686, t7690, t7755, t7757, t7989, t8103);
        let (t8111, t8119, t8643) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1289::<F>(t3, t8110, t1458, t577, t7423, t7768, t7771, t7773, t2018, t3701);
    (t8097, t8103, t8107, t8110, t8111, t8119, t8643)
}
