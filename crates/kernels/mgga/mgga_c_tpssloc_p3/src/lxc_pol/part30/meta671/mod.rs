//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta671<F: Float>(t1404: F, t7758: F, t1395: F, t7774: F, t86586: F, t86870: F, t86911: F, t86916: F, t86955: F, t86991: F, t87068: F, t87080: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91818, t91824, t92121, t92383, t92402, t92406, t92432, t92458, t92492, t92497) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2100::<F>(t1404, t7758, t1395, t7774, t86586, t86870, t86911, t86916, t86955, t86991, t87068, t87080);
    (t91818, t91824, t92121, t92383, t92402, t92406, t92432, t92458, t92492, t92497)
}
