//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta631<F: Float>(t87487: F, t22996: F, t6590: F, t23110: F, t25299: F, t81651: F, t23168: F, t25313: F, t252: F, t87230: F, t25321: F, t25284: F, t6579: F) -> (F, F, F, F, F, F, F) {
        let (t87488, t87504, t87521, t87523, t87529, t87534, t87535) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2037::<F>(t87487, t22996, t6590, t23110, t25299, t81651, t23168, t25313, t252, t87230, t25321, t25284, t6579);
    (t87488, t87504, t87521, t87523, t87529, t87534, t87535)
}
