//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1956;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1957;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta579<F: Float>(t1976: F, t5493: F, t1458: F, t7670: F, t19596: F, t2019: F, t1983: F, t7458: F, t7468: F, t1873: F, t6287: F, t652: F, t1442: F, t1774: F, t1849: F, t1869: F, t28819: F, t28822: F, t28825: F, t28829: F, t28833: F, t28837: F, t28841: F, t28843: F, t4028: F, t5450: F, t5457: F, t7451: F, t7472: F, t7681: F, t28816: F, t3: F, t20162: F, t16524: F, t7769: F, t5371: F, t7467: F, t5456: F, t576: F, t3941: F, t1401: F, t28017: F, t23880: F, t26523: F, t577: F, t7010: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28852, t28855, t28860, t28861, t28863, t28864, t28866) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1955::<F>(t1976, t5493, t1458, t7670, t19596, t2019, t1983, t7458, t7468, t1873, t6287, t652);
        let t28867 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1956::<F>(t1442, t1774, t1849, t1869, t1976, t28819, t28822, t28825, t28829, t28833, t28837, t28841, t28843, t28852, t28855, t28861, t28863, t28866, t4028, t5450, t5457, t6287, t652, t7451, t7472, t7670, t7681);
        let (t28868, t28869, t28888, t28890, t28892, t28893, t28895, t28896) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1957::<F>(t28816, t28867, t3, t1873, t20162, t16524, t7769, t5371, t7467, t5456, t576, t1458);
        let (t28899, t28904) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1958::<F>(t28896, t3941, t1873, t5493, t1401, t28017, t1458, t23880, t26523, t28868, t28888, t28890, t28892, t28895, t5456, t577, t7010);
    (t28852, t28855, t28860, t28864, t28868, t28869, t28893, t28896, t28899, t28904)
}
