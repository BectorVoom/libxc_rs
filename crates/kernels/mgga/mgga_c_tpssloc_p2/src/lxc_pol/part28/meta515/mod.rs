//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1763;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta515<F: Float>(t1388: F, t5356: F, t1351: F, t5187: F, t19735: F, t1352: F, t5286: F, t1799: F, t3698: F, t4303: F, t776: F, t1484: F, t2752: F) -> (F, F, F, F, F, F, F) {
        let (t56404, t56805, t57554, t57643, t57802, t57893, t57911) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1763::<F>(t1388, t5356, t1351, t5187, t19735, t1352, t5286, t1799, t3698, t4303, t776, t1484, t2752);
    (t56404, t56805, t57554, t57643, t57802, t57893, t57911)
}
