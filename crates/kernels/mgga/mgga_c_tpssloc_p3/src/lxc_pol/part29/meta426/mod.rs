//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta426<F: Float>(t1864: F, t645: F, t1266: F, t6534: F, t652: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F) {
        let (t22550, t22561, t22563, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1716::<F>(t1864, t645, t1266, t6534, t652, t192, t532, t1982);
    (t22550, t22561, t22563, t22573, t22574)
}
