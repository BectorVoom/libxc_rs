//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta567<F: Float>(t1068: F, t4696: F, t1597: F, t976: F, t1022: F, t3966: F, t1395: F, t671: F, t23862: F, t580: F, t23901: F, t576: F, t1404: F, t7002: F, t2029: F, t3931: F, t2022: F, t3946: F, t1372: F, t794: F, t6897: F, t6907: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60941, t61066, t61774, t66940, t80593, t80597) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2010::<F>(t1068, t4696, t1597, t976, t1022, t3966, t1395, t671, t23862, t580, t23901, t576);
        let (t80599, t80601, t80605, t80645, t80647, t80650) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2011::<F>(t1404, t7002, t2029, t3931, t2022, t3946, t1372, t794, t6897, t6907, t213, t225);
    (t60941, t61066, t61774, t66940, t80593, t80597, t80599, t80601, t80605, t80645, t80647, t80650)
}
