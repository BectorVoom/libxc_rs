//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1547;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta376<F: Float>(t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F, t1044: F, t13611: F, t248: F, t1023: F, t13975: F, t4582: F, t3121: F, t4593: F, t3041: F, t1031: F, t1612: F, t3082: F, t1025: F, t1041: F, t1046: F, t10873: F, t10883: F, t10952: F, t10965: F, t1622: F, t3039: F, t3048: F, t3117: F, t378: F, t4585: F, t4590: F, t4600: F, t4636: F) -> (F, F, F, F, F) {
        let (t14077, t14080, t14084, t14085, t14093, t14098) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1547::<F>(t3108, t4640, t1611, t3047, t3103, t4641, t1040, t4616, t1044, t13611, t248, t1023, t13975);
        let (t14099, t14103, t14107, t14120) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1548::<F>(t14098, t4582, t3121, t4593, t3041, t1031, t4616, t1612, t3082, t1025, t1041, t1046, t10873, t10883, t10952, t10965, t14077, t14080, t14084, t14085, t14093, t1622, t3039, t3048, t3117, t378, t4585, t4590, t4600, t4636);
    (t14093, t14099, t14103, t14107, t14120)
}
