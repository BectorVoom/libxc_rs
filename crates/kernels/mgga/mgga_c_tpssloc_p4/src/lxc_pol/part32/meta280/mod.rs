//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1263;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1264;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1265;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1266;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1267;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta280<F: Float>(t2109: F, t7445: F, t5: F, t1860: F, t2110: F, t7246: F, t7428: F, t7432: F, t7435: F, t7975: F, t112: F, t25: F, t265: F, t394: F, t1458: F, t2165: F, t7642: F, t1409: F, t2116: F, t40: F, t7552: F, t1419: F, t337: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1653: F, t7286: F, t7285: F, t1716: F, t2123: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t7978 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1263::<F>(t2109, t7445);
        let (t7982, t7983) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1264::<F>(t5, t1860, t2110, t7246, t7428, t7432, t7435, t7975, t7978, t112);
        let (t7989, t7992, t7997, t7998, t7999) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1265::<F>(t25, t265, t394, t1458, t2165, t7642, t1409, t2116, t40, t7552, t1419, t337, t1887, dens_threshold, rho0, zeta_threshold);
        let t8002 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1266::<F>(t1653, t7286);
        let t8003 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1267::<F>(t7285, t8002);
        let t8006 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1268::<F>(t1716, t2123);
    (t7978, t7982, t7983, t7989, t7992, t7997, t7998, t7999, t8002, t8003, t8006)
}
