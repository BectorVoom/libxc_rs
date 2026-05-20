//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta470<F: Float>(t1926: F, t3158: F, t40: F, t6722: F, t1937: F, t1929: F, t34: F, t1932: F, t1934: F, t6729: F, t131: F, t23322: F) -> (F, F, F, F, F, F, F) {
        let (t23447, t23449, t23452, t23453, t23454, t23457, t23460) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1832::<F>(t1926, t3158, t40, t6722, t1937, t1929, t34, t1932, t1934, t6729, t131, t23322);
    (t23447, t23449, t23452, t23453, t23454, t23457, t23460)
}
