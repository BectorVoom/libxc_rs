//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta579<F: Float>(t10868: F, t820: F, t3070: F, t3072: F, t10489: F, t3117: F, t1015: F, t10472: F, t42559: F, t10870: F, t3048: F, t204: F, t376: F, t1020: F, t1023: F, t248: F, t10510: F, t3109: F, t10965: F, t3053: F, t3082: F, t3094: F, t10895: F, t10952: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43198, t43200, t43206, t43211, t43214, t43216) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2144::<F>(t10868, t820, t3070, t3072, t10489, t3117, t1015, t10472, t42559, t10870, t3048, t204, t376);
        let (t43219, t43221, t43226, t43228, t43233) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2145::<F>(t1020, t1023, t248, t43216, t10510, t3109, t10965, t3053, t3082, t3094, t10895, t10952);
    (t43198, t43200, t43206, t43211, t43214, t43216, t43219, t43221, t43226, t43228, t43233)
}
