//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1362;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1363;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta302<F: Float>(t1005: F, t3082: F, t1036: F, t3094: F, t3089: F, t248: F, t2780: F, t3051: F, t1041: F, t121: F, t3061: F, t2771: F, t1008: F, t349: F, t1011: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10436, t10441, t10449, t10454, t10455, t10459) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1362::<F>(t1005, t3082, t1036, t3094, t3089, t248, t2780, t3051, t1041, t121, t3061, t2771);
        let (t10460, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1363::<F>(t1041, t10459, t1008, t349, t1011);
        let t10472 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1364::<F>(t10470, t10471);
    (t10436, t10441, t10449, t10454, t10455, t10459, t10460, t10469, t10470, t10471, t10472)
}
