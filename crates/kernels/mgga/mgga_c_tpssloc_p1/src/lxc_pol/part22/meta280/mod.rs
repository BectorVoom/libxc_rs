//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta280<F: Float>(t12923: F, t607: F, t4194: F, t3966: F, t751: F, t707: F, t157: F, t9897: F, t2371: F, t4199: F, t1409: F, t2517: F) -> (F, F, F, F, F, F, F) {
        let (t12924, t12926, t12932, t12934, t12939, t12943, t12945) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1429::<F>(t12923, t607, t4194, t3966, t751, t707, t157, t9897, t2371, t4199, t1409, t2517);
    (t12924, t12926, t12932, t12934, t12939, t12943, t12945)
}
