//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta210<F: Float>(t1406: F, t2239: F, t1454: F, t2281: F, t1472: F, t2517: F, t1409: F, t750: F, t157: F, t9897: F, t2371: F, t4199: F) -> (F, F, F, F, F, F) {
        let (t12571, t12747, t12861, t12923, t12939, t12943) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk853::<F>(t1406, t2239, t1454, t2281, t1472, t2517, t1409, t750, t157, t9897, t2371, t4199);
    (t12571, t12747, t12861, t12923, t12939, t12943)
}
