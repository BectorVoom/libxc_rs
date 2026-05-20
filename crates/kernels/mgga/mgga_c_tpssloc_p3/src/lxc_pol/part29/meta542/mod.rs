//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta542<F: Float>(t12524: F, t7769: F, t20173: F, t1458: F, t6534: F, t3941: F, t1873: F, t4072: F, t3938: F, t7467: F, t671: F, t1401: F, t26135: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26539, t26541, t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1936::<F>(t12524, t7769, t20173, t1458, t6534, t3941, t1873, t4072, t3938, t7467, t671, t1401, t26135);
    (t26539, t26541, t26542, t26544, t26545, t26547, t26549, t26550, t26552, t26554)
}
