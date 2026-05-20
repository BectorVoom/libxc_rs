//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1750;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta427<F: Float>(t19153: F, t6252: F, t11889: F, t1215: F, t5079: F, t6260: F, t11888: F, t11904: F, t11907: F, t11914: F, t1244: F, t15027: F, t15032: F, t15245: F, t1756: F, t19123: F, t19129: F, t19131: F, t19139: F, t19142: F, t19146: F, t3604: F, t3610: F, t3624: F, t5064: F, t5069: F, t5080: F, t5084: F, t6253: F, t6261: F, t6263: F, t11883: F, t1751: F, t5011: F, t1246: F, t6238: F, t19145: F, t3612: F, t1734: F, t5052: F, t1235: F, t6218: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19154, t19156, t19157, t19160, t19164) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1750::<F>(t19153, t6252, t11889, t1215, t5079, t6260, t11888, t11904, t11907, t11914, t1244, t15027, t15032, t15245, t1756, t19123, t19129, t19131, t19139, t19142, t19146, t3604, t3610, t3624, t5064, t5069, t5080, t5084, t6253, t6261, t6263);
        let (t19165, t19166, t19170, t19174, t19176, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1751::<F>(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
    (t19154, t19156, t19157, t19160, t19164, t19165, t19166, t19170, t19174, t19176, t19180, t19189)
}
