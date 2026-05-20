//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta281<F: Float>(t12945: F, t707: F, t3966: F, t75: F, t78: F, t1484: F, t212: F, t9523: F, t2586: F, t213: F, t4119: F, t2570: F, t67: F) -> (F, F, F, F, F, F, F, F) {
        let (t12946, t12950, t12961, t12984, t12985, t12986, t12988, t12997) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1430::<F>(t12945, t707, t3966, t75, t78, t1484, t212, t9523, t2586, t213, t4119, t2570, t67);
    (t12946, t12950, t12961, t12984, t12985, t12986, t12988, t12997)
}
