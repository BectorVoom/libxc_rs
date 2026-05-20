//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta400<F: Float>(t2940: F, t4498: F, t2925: F, t4488: F, t959: F, t1634: F, t3175: F, t10165: F, t1065: F, t4693: F, t3174: F, t2970: F, t4343: F) -> (F, F, F, F, F, F) {
        let (t13731, t13732, t13734, t13736, t13743, t13748) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1796::<F>(t2940, t4498, t2925, t4488, t959, t1634, t3175, t10165, t1065, t4693, t3174, t2970, t4343);
    (t13731, t13732, t13734, t13736, t13743, t13748)
}
