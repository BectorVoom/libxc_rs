//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta488<F: Float>(t22588: F, t23861: F, t3: F, t112: F, t7002: F, t111: F, t2022: F, t12521: F, t1873: F, t12524: F, t7015: F, t3938: F, t6534: F, t16535: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2319: F, t577: F, t7010: F) -> (F, F, F, F, F, F, F) {
        let (t23862, t23863, t23877, t23880, t23886, t23888, t23890) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1871::<F>(t22588, t23861, t3, t112, t7002, t111, t2022, t12521, t1873, t12524, t7015, t3938, t6534);
        let (t23893, t23896, t23901) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1872::<F>(t16535, t1873, t6534, t671, t3941, t2363, t1401, t22479, t2319, t23862, t23877, t23880, t23886, t23888, t23890, t577, t7010);
    (t23862, t23863, t23877, t23880, t23893, t23896, t23901)
}
