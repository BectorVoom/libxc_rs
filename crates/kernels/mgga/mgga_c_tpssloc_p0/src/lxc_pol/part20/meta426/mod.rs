//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta426<F: Float>(t1569: F, t2880: F, t2862: F, t4437: F, t2888: F, t4433: F, t931: F, t10813: F, t1568: F, t4472: F, t950: F, t1581: F, t2924: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14429, t14432, t14435, t14436, t14439, t14442, t14443, t14450, t14453) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1842::<F>(t1569, t2880, t2862, t4437, t2888, t4433, t931, t10813, t1568, t4472, t950, t1581, t2924);
    (t14429, t14432, t14435, t14436, t14439, t14442, t14443, t14450, t14453)
}
