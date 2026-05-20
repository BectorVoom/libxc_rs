//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta248<F: Float>(t1174: F, t18454: F, t11539: F, t6119: F, t4889: F, t4896: F, t11570: F, t5392: F, t1171: F, t6109: F, t6011: F, t699: F) -> (F, F, F, F, F, F, F) {
        let (t18455, t18457, t18458, t18460, t18469, t18489, t18494) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk907::<F>(t1174, t18454, t11539, t6119, t4889, t4896, t11570, t5392, t1171, t6109, t6011, t699);
    (t18455, t18457, t18458, t18460, t18469, t18489, t18494)
}
