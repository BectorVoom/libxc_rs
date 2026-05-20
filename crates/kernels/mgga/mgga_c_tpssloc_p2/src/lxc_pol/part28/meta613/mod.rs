//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta613<F: Float>(t26421: F, t26446: F, t3734: F, t90591: F, t22751: F, t26389: F, t1992: F, t22897: F, t3792: F, t90870: F, t26467: F, t6914: F, t26426: F, t81046: F, t22690: F, t7732: F, t81195: F, t16413: F, t1985: F, t1998: F, t214: F, t16248: F, t22833: F, t16383: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91052, t91064, t91074, t91076) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927::<F>(t26421, t26446, t3734, t90591, t22751, t26389, t1992, t22897, t3792, t90870, t26467, t6914);
        let (t91078, t91081, t91091, t91094, t91096) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1928::<F>(t26426, t81046, t22690, t7732, t81195, t16413, t1985, t1998, t214, t16248, t22833, t16383);
    (t91052, t91064, t91074, t91076, t91078, t91081, t91091, t91094, t91096)
}
