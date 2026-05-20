//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2445;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta650<F: Float>(t10189: F, t3008: F, t4509: F, t13797: F, t984: F, t10216: F, t343: F, t3152: F, t698: F, t973: F, t10870: F, t3117: F, t2955: F, t3158: F, t10383: F, t964: F, t1020: F, t10508: F, t248: F, t3121: F, t10868: F, t820: F, t3070: F, t3072: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43057, t43065, t43069, t43070, t43110, t43114) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2445::<F>(t10189, t3008, t4509, t13797, t984, t10216, t343, t3152, t698, t973, t10870, t3117);
        let (t43155, t43157, t43161, t43198, t43200) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446::<F>(t2955, t3158, t10383, t964, t1020, t10508, t248, t3121, t10868, t820, t3070, t3072);
    (t43057, t43065, t43069, t43070, t43110, t43114, t43155, t43157, t43161, t43198, t43200)
}
