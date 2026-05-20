//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2167;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta632<F: Float>(t15892: F, t2535: F, t2528: F, t40225: F, t15921: F, t588: F, t15971: F, t12364: F, t5234: F, t1354: F, t12365: F, t5289: F) -> (F, F, F, F, F, F, F, F) {
        let (t54470, t54472, t54473, t54475, t54478, t54532, t54534, t54555) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2167::<F>(t15892, t2535, t2528, t40225, t15921, t588, t15971, t12364, t5234, t1354, t12365, t5289);
    (t54470, t54472, t54473, t54475, t54478, t54532, t54534, t54555)
}
