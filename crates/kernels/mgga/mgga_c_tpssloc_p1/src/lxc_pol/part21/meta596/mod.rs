//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta596<F: Float>(t3014: F, t343: F, t3475: F, t460: F, t253: F, t254: F, t4540: F, t382: F, t1458: F, t649: F, t1453: F, t666: F) -> (F, F, F, F, F, F, F) {
        let (t23547, t24705, t25168, t25608, t25757, t26114, t26129) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2347::<F>(t3014, t343, t3475, t460, t253, t254, t4540, t382, t1458, t649, t1453, t666);
    (t23547, t24705, t25168, t25608, t25757, t26114, t26129)
}
