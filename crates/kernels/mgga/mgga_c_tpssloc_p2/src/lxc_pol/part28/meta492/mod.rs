//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta492<F: Float>(t1985: F, t26351: F, t1842: F, t3886: F, t1385: F, t22635: F, t1992: F, t6883: F, t7697: F, t22897: F, t5336: F, t22751: F, t7733: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26352, t26354, t26355, t26356, t26357, t26361, t26378, t26379, t26381) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1707::<F>(t1985, t26351, t1842, t3886, t1385, t22635, t1992, t6883, t7697, t22897, t5336, t22751, t7733);
    (t26352, t26354, t26355, t26356, t26357, t26361, t26378, t26379, t26381)
}
