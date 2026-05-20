//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta294<F: Float>(t10969: F, t61: F, t10305: F, t248: F, t135: F, t3142: F, t973: F, t3147: F, t9258: F, t998: F, t974: F, t3152: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10970, t10972, t10981, t10982, t10984, t10985, t10987, t10988, t10993) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1508::<F>(t10969, t61, t10305, t248, t135, t3142, t973, t3147, t9258, t998, t974, t3152);
    (t10970, t10972, t10981, t10982, t10984, t10985, t10987, t10988, t10993)
}
