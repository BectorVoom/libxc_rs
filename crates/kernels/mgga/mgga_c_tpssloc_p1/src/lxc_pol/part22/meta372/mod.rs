//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1624;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1625;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta372<F: Float>(t1022: F, t10482: F, t17670: F, t4582: F, t1539: F, t4650: F, t3071: F, t5867: F, t884: F, t10390: F, t1041: F, t10480: F, t10904: F, t13995: F, t14000: F, t14027: F, t17643: F, t17649: F, t17656: F, t17660: F, t17662: F, t17668: F, t3070: F, t4575: F, t5875: F, t5909: F, t5392: F, t607: F, t14172: F, t1409: F, t3966: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17671, t17672, t17673, t17676, t17677, t17680, t17681, t17684) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1624::<F>(t1022, t10482, t17670, t4582, t1539, t4650, t3071, t5867, t884, t10390, t1041, t10480, t10904, t13995, t14000, t14027, t17643, t17649, t17656, t17660, t17662, t17668, t3070, t4575, t5875, t5909);
        let t17686 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1625::<F>(t5392, t607);
        let (t17687, t17688, t17691) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1626::<F>(t14172, t17686, t4582, t1409, t3966);
    (t17671, t17672, t17673, t17676, t17677, t17680, t17681, t17684, t17686, t17687, t17688, t17691)
}
