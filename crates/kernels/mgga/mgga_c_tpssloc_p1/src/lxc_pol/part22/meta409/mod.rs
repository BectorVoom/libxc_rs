//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1709;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta409<F: Float>(t18554: F, t4934: F, t1178: F, t16558: F, t1177: F, t1184: F, t460: F, t6138: F, t11556: F, t1174: F, t1187: F, t15401: F, t15405: F, t15422: F, t18321: F, t18536: F, t18543: F, t18546: F, t18550: F, t3447: F, t4889: F, t4913: F, t4931: F, t18442: F, t18473: F, t18535: F, t225: F, t68: F, t484: F, t18215: F, t3440: F, t18211: F, t1653: F, t5012: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18555, t18558, t18559, t18563, t18564, t18569) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1709::<F>(t18554, t4934, t1178, t16558, t1177, t1184, t460, t6138, t11556, t1174, t1187, t15401, t15405, t15422, t18321, t18536, t18543, t18546, t18550, t3447, t4889, t4913, t4931);
        let (t18571, t18572, t18573, t18574, t18577, t18580, t18583) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1710::<F>(t18442, t18473, t18535, t18569, t225, t68, t484, t18215, t3440, t18211, t1653, t5012);
    (t18555, t18558, t18559, t18563, t18564, t18571, t18572, t18573, t18574, t18577, t18580, t18583)
}
