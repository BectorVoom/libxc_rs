//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta70 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk477;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk478;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk479;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk480;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk481;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk482;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk483;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta70<F: Float>(t1365: F, t241: F, t67: F, t1307: F, t820: F, t1315: F, t1327: F, t1329: F, t1333: F, t1341: F, t1354: F, t1360: F, t1363: F, t559: F, t539: F, t225: F, t563: F, t566: F, t68: F, t1338: F, t562: F, t1352: F, t553: F, t1332: F, t1336: F, t544: F, t564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1367 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk477::<F>(t1365, t241, t67);
        let t1369 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk478::<F>(t1307, t1367, t820);
        let t1372 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk479::<F>(t1315, t1327, t1329, t1333, t1341, t1354, t1360, t1363, t1369, t559);
        let (t1373, t1375) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk480::<F>(t1372, t539, t225, t563);
        let (t1376, t1377) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk481::<F>(t566);
        let t1378 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk482::<F>(t1377, t68);
        let t1380 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk483::<F>(t1338, t562);
        let (t1381, t1383, t1385) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk484::<F>(t1352, t1380, t1372, t553, t1332, t1336, t544, t564);
    (t1367, t1369, t1372, t1373, t1375, t1376, t1377, t1378, t1380, t1381, t1383, t1385)
}
