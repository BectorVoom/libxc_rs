//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta532<F: Float>(t12250: F, t3850: F, t10021: F, t154: F, t59: F, t3749: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t12227: F, t9577: F, t116: F, t557: F, t212: F, t2586: F, t3734: F, t12225: F, t3719: F, t12222: F, t16081: F, t1314: F, t9534: F, t1307: F, t133: F, t6600: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40335, t40341, t40343, t40344, t40347, t40350, t40351) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2067::<F>(t12250, t3850, t10021, t154, t59, t3749, t598, t535, t795, t215, t39933, t12227, t9577);
        let (t40356, t40360, t40366, t40369, t40372) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2068::<F>(t116, t557, t212, t2586, t3734, t12225, t3719, t12222, t16081, t1314, t9534, t1307, t133, t6600);
    (t40335, t40341, t40343, t40344, t40347, t40350, t40351, t40356, t40360, t40366, t40369, t40372)
}
