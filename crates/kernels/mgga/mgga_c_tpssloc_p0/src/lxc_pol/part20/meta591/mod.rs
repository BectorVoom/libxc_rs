//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta591<F: Float>(t11277: F, t3307: F, t11285: F, t3395: F, t43776: F, t43819: F, t3311: F, t409: F, t3314: F, t11399: F, t3403: F, t11352: F, t3351: F) -> (F, F, F, F, F, F, F, F) {
        let (t43976, t43984, t44027, t44053, t44075, t44077, t44106, t44131) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2170::<F>(t11277, t3307, t11285, t3395, t43776, t43819, t3311, t409, t3314, t11399, t3403, t11352, t3351);
    (t43976, t43984, t44027, t44053, t44075, t44077, t44106, t44131)
}
