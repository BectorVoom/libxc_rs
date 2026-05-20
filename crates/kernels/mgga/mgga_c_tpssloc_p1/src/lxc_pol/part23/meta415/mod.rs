//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta415<F: Float>(t120: F, t20800: F, t20904: F, t41414: F, t20949: F, t2697: F, t20882: F, t9638: F, t13258: F, t20988: F, t20887: F, t20969: F, t2639: F) -> (F, F, F, F, F, F, F) {
        let (t67644, t67660, t67675, t67690, t67692, t67729, t67735) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1233::<F>(t120, t20800, t20904, t41414, t20949, t2697, t20882, t9638, t13258, t20988, t20887, t20969, t2639);
    (t67644, t67660, t67675, t67690, t67692, t67729, t67735)
}
