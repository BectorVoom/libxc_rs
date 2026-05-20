//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta622<F: Float>(t2240: F, t2251: F, t2261: F, t43: F, t2267: F, t614: F, t38: F, t9287: F, t835: F, t1862: F, t2244: F, t39054: F, t6489: F) -> (F, F, F, F, F, F, F) {
        let (t83778, t83788, t83791, t83796, t83803, t83814, t83827) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2101::<F>(t2240, t2251, t2261, t43, t2267, t614, t38, t9287, t835, t1862, t2244, t39054, t6489);
    (t83778, t83788, t83791, t83796, t83803, t83814, t83827)
}
