//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta408<F: Float>(t4889: F, t4916: F, t1653: F, t7319: F, t4919: F, t15293: F, t4928: F, t8034: F, t4934: F, t1184: F, t460: F, t6144: F) -> (F, F, F, F, F, F, F) {
        let (t18536, t18542, t18543, t18546, t18549, t18550, t18554) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1708::<F>(t4889, t4916, t1653, t7319, t4919, t15293, t4928, t8034, t4934, t1184, t460, t6144);
    (t18536, t18542, t18543, t18546, t18549, t18550, t18554)
}
