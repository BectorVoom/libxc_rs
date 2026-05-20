//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta526<F: Float>(t26231: F, t26251: F, t26255: F, t26266: F, t26361: F, t26393: F, t26406: F, t26429: F, t26127: F, t2165: F, t4072: F, t671: F, t8103: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27290, t27293) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1860::<F>(t26231, t26251, t26255, t26266, t26361, t26393, t26406, t26429, t26127, t2165, t4072, t671, t8103);
    (t27012, t27019, t27022, t27027, t27067, t27082, t27088, t27096, t27166, t27290, t27293)
}
