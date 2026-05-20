//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta549<F: Float>(t1864: F, t2241: F, t608: F, t9231: F, t645: F, t6509: F, t2307: F, t2240: F, t2251: F, t22573: F, t6875: F, t24486: F, t576: F) -> (F, F, F, F, F, F, F) {
        let (t83718, t83722, t83728, t83737, t83778, t83886, t84031) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1819::<F>(t1864, t2241, t608, t9231, t645, t6509, t2307, t2240, t2251, t22573, t6875, t24486, t576);
    (t83718, t83722, t83728, t83737, t83778, t83886, t84031)
}
