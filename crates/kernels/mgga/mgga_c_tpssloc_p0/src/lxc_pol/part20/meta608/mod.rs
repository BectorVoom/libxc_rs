//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta608<F: Float>(t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F, t11669: F, t3577: F, t11779: F, t11677: F, t11907: F, t11769: F, t13969: F, t3515: F) -> (F, F, F, F, F, F, F, F) {
        let (t45113, t45114, t45119, t45124, t45126, t45128, t45134, t45148) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2193::<F>(t3575, t42386, t11888, t11914, t11784, t820, t11669, t3577, t11779, t11677, t11907, t11769, t13969, t3515);
    (t45113, t45114, t45119, t45124, t45126, t45128, t45134, t45148)
}
