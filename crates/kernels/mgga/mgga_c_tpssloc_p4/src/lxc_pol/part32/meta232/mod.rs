//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1054;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1055;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1056;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta232<F: Float>(t457: F, t6138: F, t460: F, t974: F, t1714: F, t1174: F, t1710: F, t1717: F, t3430: F, t3447: F, t463: F, t4887: F, t4889: F, t4897: F, t4917: F, t6109: F, t6120: F, t6123: F, t6127: F, t6131: F) -> (F, F, F, F, F, F) {
        let (t6139, t6140) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1054::<F>(t457, t6138, t460);
        let (t6141, t6144) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1055::<F>(t6140, t974, t1714);
        let (t6145, t6146) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1056::<F>(t457, t6144, t460);
        let t6150 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1057::<F>(t6146, t974, t1174, t1710, t1717, t3430, t3447, t463, t4887, t4889, t4897, t4917, t6109, t6120, t6123, t6127, t6131, t6141);
    (t6139, t6140, t6144, t6145, t6146, t6150)
}
