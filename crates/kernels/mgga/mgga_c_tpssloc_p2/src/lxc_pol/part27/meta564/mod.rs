//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta564<F: Float>(t1851: F, t2319: F, t2363: F, t576: F, t4025: F, t671: F, t1441: F, t1799: F, t3914: F, t1388: F, t5187: F, t1307: F, t5356: F) -> (F, F, F, F, F, F, F) {
        let (t55405, t55571, t55934, t55962, t56120, t56194, t56198) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2007::<F>(t1851, t2319, t2363, t576, t4025, t671, t1441, t1799, t3914, t1388, t5187, t1307, t5356);
    (t55405, t55571, t55934, t55962, t56120, t56194, t56198)
}
