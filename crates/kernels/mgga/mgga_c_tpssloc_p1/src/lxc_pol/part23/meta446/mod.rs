//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta446<F: Float>(t20576: F, t3726: F, t16081: F, t20586: F, t20602: F, t225: F, t20420: F, t20672: F, t20670: F, t1834: F, t6414: F, t20553: F, t562: F) -> (F, F, F, F, F, F, F, F) {
        let (t74747, t74756, t74849, t74860, t74908, t74930, t74937, t74949) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1291::<F>(t20576, t3726, t16081, t20586, t20602, t225, t20420, t20672, t20670, t1834, t6414, t20553, t562);
    (t74747, t74756, t74849, t74860, t74908, t74930, t74937, t74949)
}
