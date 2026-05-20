//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1956;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta578<F: Float>(t29560: F, t1932: F, t2133: F, t7573: F, t8027: F, t1737: F, t2136: F, t24681: F, t24704: F, t27578: F, t27592: F, t27599: F, t27609: F, t27614: F, t6203: F, t6211: F, t7345: F, t337: F, t5415: F, t131: F, t475: F, t6218: F, t68: F, t7328: F, t1730: F, t8048: F, t2139: F, t6163: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29561, t29562, t29563, t29569, t29580) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1956::<F>(t29560, t1932, t2133, t7573, t8027, t1737, t2136, t24681, t24704, t27578, t27592, t27599, t27609, t27614, t6203, t6211, t7345);
        let (t29584, t29585, t29593, t29594, t29597, t29600, t29601) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1957::<F>(t337, t5415, t131, t475, t6218, t68, t7328, t1730, t8048, t2139, t6163, t471);
    (t29561, t29562, t29563, t29569, t29580, t29584, t29585, t29593, t29594, t29597, t29600, t29601)
}
