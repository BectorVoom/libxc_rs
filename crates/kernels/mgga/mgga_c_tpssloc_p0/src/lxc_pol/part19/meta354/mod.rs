//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta354<F: Float>(t270: F, t276: F, t39267: F, t2799: F, t2807: F, t2798: F, t273: F, t2815: F, t10588: F, t896: F, t10595: F, t10599: F) -> (F, F, F, F, F, F, F, F) {
        let (t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1283::<F>(t270, t276, t39267, t2799, t2807, t2798, t273, t2815, t10588, t896, t10595, t10599);
    (t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957)
}
