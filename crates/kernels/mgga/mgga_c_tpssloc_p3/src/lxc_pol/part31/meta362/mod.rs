//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta362<F: Float>(t1354: F, t16288: F, t12211: F, t5223: F, t3804: F, t820: F, t1351: F, t1824: F, t3792: F, t12345: F, t1831: F, t1362: F, t16060: F) -> (F, F, F, F, F, F, F) {
        let (t16290, t16294, t16305, t16306, t16311, t16317, t16321) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1283::<F>(t1354, t16288, t12211, t5223, t3804, t820, t1351, t1824, t3792, t12345, t1831, t1362, t16060);
    (t16290, t16294, t16305, t16306, t16311, t16317, t16321)
}
