//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta287 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1587;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta287<F: Float>(t10422: F, t3072: F, t3070: F, t1005: F, t3082: F, t1036: F, t3094: F, t3089: F, t248: F, t2780: F, t3051: F, t1041: F, t121: F, t3061: F, t2771: F, t1008: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10423, t10424, t10436, t10441, t10449, t10454, t10455) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1587::<F>(t10422, t3072, t3070, t1005, t3082, t1036, t3094, t3089, t248, t2780, t3051, t1041);
        let (t10457, t10459, t10460, t10468, t10469) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1588::<F>(t121, t3061, t248, t2771, t1041, t1008);
    (t10423, t10424, t10436, t10441, t10449, t10454, t10455, t10457, t10459, t10460, t10468, t10469)
}
