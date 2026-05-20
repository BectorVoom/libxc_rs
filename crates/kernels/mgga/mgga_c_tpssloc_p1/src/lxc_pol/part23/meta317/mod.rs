//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1075;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta317<F: Float>(t22055: F, t3440: F, t20234: F, t3441: F, t1177: F, t21745: F, t4900: F, t15390: F, t18469: F, t18416: F, t4904: F, t18409: F, t4919: F, t18427: F, t11547: F, t11546: F, t1174: F, t15265: F, t1710: F, t1717: F, t18321: F, t22035: F, t22041: F, t22047: F, t22052: F, t3447: F, t4889: F, t6120: F, t6141: F, t6147: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22056, t22059, t22060, t22063, t22066, t22069, t22072) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1075::<F>(t22055, t3440, t20234, t3441, t1177, t21745, t4900, t15390, t18469, t18416, t4904, t18409, t4919);
        let (t22075, t22081, t22082, t22085) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1076::<F>(t18427, t4919, t11547, t20234, t11546, t1174, t15265, t1710, t1717, t18321, t22035, t22041, t22047, t22052, t22056, t22060, t22063, t22066, t22069, t22072, t3447, t4889, t6120, t6141, t6147);
    (t22056, t22059, t22060, t22063, t22066, t22069, t22072, t22075, t22081, t22082, t22085)
}
