//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1924;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1925;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta496<F: Float>(t10377: F, t10385: F, t10480: F, t10876: F, t10883: F, t14508: F, t14511: F, t17612: F, t17616: F, t21393: F, t21398: F, t21405: F, t21483: F, t21487: F, t21490: F, t21493: F, t3130: F, t378: F, t5875: F, t5880: F, t973: F, t1616: F, t17712: F, t4582: F, t1409: F, t5398: F, t4588: F, t10970: F, t21130: F, t248: F, t5681: F, t3071: F, t1539: F, t5873: F, t10403: F, t1041: F, t13966: F, t13995: F, t17621: F, t17625: F, t17656: F, t17660: F, t17662: F, t17668: F, t3039: F, t3070: F, t5909: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t21498 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1924::<F>(t10377, t10385, t10480, t10876, t10883, t14508, t14511, t17612, t17616, t21393, t21398, t21405, t21483, t21487, t21490, t21493, t3130, t378, t5875, t5880, t973);
        let (t21502, t21503, t21510) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1925::<F>(t1616, t17712, t4582, t1409, t5398);
        let (t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1926::<F>(t21510, t4588, t4582, t10970, t21130, t248, t1616, t5681, t3071, t1539, t5873, t10403, t1041, t13966, t13995, t17621, t17625, t17656, t17660, t17662, t17668, t21503, t3039, t3070, t5909);
    (t21498, t21502, t21503, t21510, t21511, t21512, t21516, t21519, t21520, t21525, t21526, t21529)
}
