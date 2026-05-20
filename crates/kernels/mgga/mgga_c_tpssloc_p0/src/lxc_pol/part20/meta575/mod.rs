//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta575<F: Float>(t10224: F, t2995: F, t973: F, t10228: F, t2960: F, t10225: F, t10213: F, t135: F, t10218: F, t10236: F, t10913: F, t41961: F) -> (F, F, F, F, F, F, F) {
        let (t42962, t42964, t42968, t42972, t42974, t42985, t43002) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2138::<F>(t10224, t2995, t973, t10228, t2960, t10225, t10213, t135, t10218, t10236, t10913, t41961);
    (t42962, t42964, t42968, t42972, t42974, t42985, t43002)
}
