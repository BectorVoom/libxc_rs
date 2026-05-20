//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1958;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta428<F: Float>(t11889: F, t3507: F, t1755: F, t15018: F, t3612: F, t5075: F, t5079: F, t1706: F, t3428: F, t1184: F, t460: F, t4928: F, t4934: F, t1714: F, t3469: F, t1178: F, t12606: F, t1177: F, t135: F, t457: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15247, t15248, t15253, t15257, t15265, t15268) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1958::<F>(t11889, t3507, t1755, t15018, t3612, t5075, t5079, t1706, t3428, t1184, t460, t4928);
        let (t15269, t15273, t15274, t15277, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1959::<F>(t15268, t4934, t1714, t3469, t460, t1178, t12606, t1177, t135, t457);
    (t15247, t15248, t15253, t15257, t15265, t15268, t15269, t15273, t15274, t15277, t15278, t15281)
}
