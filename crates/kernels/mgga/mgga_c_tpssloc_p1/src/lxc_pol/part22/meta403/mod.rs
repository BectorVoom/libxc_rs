//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1700;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta403<F: Float>(t15395: F, t18206: F, t15338: F, t4904: F, t3447: F, t3431: F, t6126: F, t1174: F, t6130: F, t11539: F, t6119: F, t4889: F, t4896: F, t18215: F, t4900: F, t11570: F, t5392: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18443, t18446, t18447, t18451, t18452, t18454, t18455, t18457, t18458, t18460) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1700::<F>(t15395, t18206, t15338, t4904, t3447, t3431, t6126, t1174, t6130, t11539, t6119, t4889, t4896);
        let (t18466, t18469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1701::<F>(t18215, t4900, t11570, t5392);
    (t18443, t18446, t18447, t18451, t18452, t18454, t18455, t18457, t18458, t18460, t18466, t18469)
}
