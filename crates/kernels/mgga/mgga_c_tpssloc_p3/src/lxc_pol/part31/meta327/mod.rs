//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1222;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta327<F: Float>(t12858: F, t763: F, t1472: F, t2517: F, t4303: F, t870: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F, t3966: F, t751: F, t707: F, t157: F, t9897: F, t2371: F, t4199: F, t1484: F, t212: F, t9523: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12860, t12861, t12895, t12922, t12926) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1222::<F>(t12858, t763, t1472, t2517, t4303, t870, t2430, t4205, t1409, t750, t607, t4194);
        let (t12934, t12939, t12943, t12946, t12984, t12985) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1223::<F>(t3966, t751, t707, t157, t9897, t2371, t4199, t1409, t2517, t1484, t212, t9523);
    (t12860, t12861, t12895, t12922, t12926, t12934, t12939, t12943, t12946, t12984, t12985)
}
