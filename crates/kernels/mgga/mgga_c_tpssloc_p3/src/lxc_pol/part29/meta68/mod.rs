//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta68 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk459;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk460;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk461;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk462;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk463;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk464;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk465;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk466;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta68<F: Float>(t1323: F, t562: F, t541: F, t801: F, t119: F, t1307: F, t210: F, t225: F, t554: F, t544: F, t68: F, t551: F, t236: F, t240: F, t241: F, t557: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1324, t1327, t1329) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk459::<F>(t1323, t562, t541, t801, t119, t1307, t210);
        let t1332 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk460::<F>(t1323, t225);
        let (t1333, t1336) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk461::<F>(t1332, t554, t544, t68);
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk462::<F>(t551);
        let t1339 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk463::<F>(t1338, t236);
        let t1340 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk464::<F>(t1339, t240);
        let t1341 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk465::<F>(t1336, t1340);
        let t1343 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk466::<F>(t241, t557, t67);
    (t1324, t1327, t1329, t1332, t1333, t1336, t1337, t1338, t1339, t1340, t1341, t1343)
}
