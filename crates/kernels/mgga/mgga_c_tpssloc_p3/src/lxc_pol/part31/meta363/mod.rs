//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta363<F: Float>(t12339: F, t1831: F, t3866: F, t5314: F, t3865: F, t5234: F, t1369: F, t12189: F, t1811: F, t1358: F, t5231: F, t1815: F, t3862: F) -> (F, F, F, F, F, F, F) {
        let (t16325, t16331, t16336, t16338, t16341, t16346, t16350) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1284::<F>(t12339, t1831, t3866, t5314, t3865, t5234, t1369, t12189, t1811, t1358, t5231, t1815, t3862);
    (t16325, t16331, t16336, t16338, t16341, t16346, t16350)
}
