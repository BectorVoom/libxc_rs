//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1702;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta404<F: Float>(t11569: F, t18469: F, t1180: F, t15284: F, t15287: F, t15300: F, t15307: F, t18321: F, t18443: F, t18447: F, t18452: F, t18455: F, t18458: F, t18460: F, t18466: F, t3447: F, t4889: F, t4937: F, t18211: F, t4900: F, t15382: F, t15390: F, t1171: F, t6109: F, t6011: F, t699: F) -> (F, F, F, F, F, F) {
        let (t18470, t18473) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1702::<F>(t11569, t18469, t1180, t15284, t15287, t15300, t15307, t18321, t18443, t18447, t18452, t18455, t18458, t18460, t18466, t3447, t4889, t4937);
        let (t18475, t18484, t18489, t18494) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1703::<F>(t18211, t4900, t15382, t15390, t1171, t6109, t6011, t699);
    (t18470, t18473, t18475, t18484, t18489, t18494)
}
