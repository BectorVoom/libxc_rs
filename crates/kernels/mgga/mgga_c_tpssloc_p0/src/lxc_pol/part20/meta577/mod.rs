//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2140;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta577<F: Float>(t2986: F, t2990: F, t43057: F, t10325: F, t2987: F, t3008: F, t4509: F, t13797: F, t984: F, t10216: F, t343: F, t9288: F, t10236: F, t10427: F, t13969: F, t3130: F, t10432: F, t3039: F, t10943: F, t135: F, t973: F, t3152: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43059, t43061, t43065, t43069, t43070, t43071) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2140::<F>(t2986, t2990, t43057, t10325, t2987, t3008, t4509, t13797, t984, t10216, t343, t9288);
        let (t43075, t43094, t43097, t43103, t43110) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2141::<F>(t10236, t9288, t10427, t13969, t3130, t10432, t3039, t10943, t135, t973, t3152, t698);
    (t43059, t43061, t43065, t43069, t43070, t43071, t43075, t43094, t43097, t43103, t43110)
}
