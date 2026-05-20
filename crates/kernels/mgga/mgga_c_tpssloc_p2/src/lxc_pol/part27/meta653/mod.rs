//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2280;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta653<F: Float>(t24996: F, t90442: F, t24995: F, t34475: F, t5308: F, t1983: F, t26503: F, t6999: F, t12823: F, t7468: F, t26003: F, t4034: F, t26351: F, t6883: F, t1992: F, t26355: F, t80650: F, t22635: F, t26354: F, t3911: F, t22751: F, t26186: F, t26190: F, t26356: F, t6914: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90444, t90447, t90450, t90454, t90456) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2280::<F>(t24996, t90442, t24995, t34475, t5308, t1983, t26503, t6999, t12823, t7468, t26003, t4034);
        let (t90460, t90462, t90466, t90469, t90471, t90472) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2281::<F>(t26351, t6883, t1992, t26355, t80650, t22635, t26354, t3911, t22751, t26186, t26190, t26356, t6914);
    (t90444, t90447, t90450, t90454, t90456, t90460, t90462, t90466, t90469, t90471, t90472)
}
