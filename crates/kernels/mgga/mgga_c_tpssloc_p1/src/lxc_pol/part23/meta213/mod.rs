//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta213<F: Float>(t13123: F, t2375: F, t1512: F, t9671: F, t2644: F, t820: F, t1509: F, t2632: F, t1500: F, t2693: F, t2642: F, t4166: F) -> (F, F, F, F, F, F) {
        let (t13124, t13182, t13222, t13228, t13234, t13251) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk857::<F>(t13123, t2375, t1512, t9671, t2644, t820, t1509, t2632, t1500, t2693, t2642, t4166);
    (t13124, t13182, t13222, t13228, t13234, t13251)
}
