//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1924;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta519<F: Float>(t23384: F, t7604: F, t1615: F, t6768: F, t1060: F, t2987: F, t4343: F, t4338: F, t4509: F, t4640: F, t6754: F, t1611: F, t6764: F) -> (F, F, F, F, F, F, F) {
        let (t25563, t25567, t25568, t25571, t25574, t25577, t25580) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1924::<F>(t23384, t7604, t1615, t6768, t1060, t2987, t4343, t4338, t4509, t4640, t6754, t1611, t6764);
    (t25563, t25567, t25568, t25571, t25574, t25577, t25580)
}
