//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1485;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1486;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta311<F: Float>(t1687: F, t3375: F, t1128: F, t4794: F, t1675: F, t3356: F, t11352: F, t1682: F, t14722: F, t14704: F, t3331: F, t3403: F, t4857: F, t11285: F, t1694: F, t15026: F, t3623: F, t1706: F, t3428: F, t135: F, t457: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15136, t15141, t15146) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1485::<F>(t1687, t3375, t1128, t4794, t1675, t3356);
        let (t15171, t15194, t15195, t15207, t15218, t15225, t15245) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1486::<F>(t11352, t1682, t14722, t14704, t1675, t3331, t3403, t4857, t11285, t1694, t15026, t3623);
        let (t15265, t15281) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1487::<F>(t1706, t3428, t135, t457);
    (t15136, t15141, t15146, t15171, t15194, t15195, t15207, t15218, t15225, t15245, t15265, t15281)
}
