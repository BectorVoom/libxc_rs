//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1725;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta419<F: Float>(t6505: F, t6509: F, t2235: F, t608: F, t33: F, t6504: F, t2240: F, t641: F, t645: F, t72: F, t2307: F, t79: F, t2244: F, t605: F, t2251: F, t6489: F, t9239: F, t2241: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22516, t22519, t22522, t22523, t22527, t22530) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1725::<F>(t6505, t6509, t2235, t608, t33, t6504, t2240, t641, t645, t72, t2307, t79);
        let (t22531, t22534, t22537, t22544, t22546, t22549, t22550) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1726::<F>(t22530, t72, t2244, t605, t2251, t6489, t9239, t2241, t79, t2240, t608, t1864, t645);
    (t22516, t22519, t22522, t22523, t22527, t22531, t22534, t22537, t22544, t22546, t22549, t22550)
}
