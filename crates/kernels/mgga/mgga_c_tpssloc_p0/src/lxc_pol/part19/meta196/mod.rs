//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk861;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta196<F: Float>(t10231: F, t2981: F, t973: F, t4509: F, t984: F, t2770: F, t343: F, t2244: F, t2987: F, t3008: F, t2990: F, t2250: F, t2989: F, t2988: F, t2775: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10232, t10233, t10235, t10236, t10237, t10238, t10241, t10242, t10245) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk861::<F>(t10231, t2981, t973, t4509, t984, t2770, t343, t2244, t2987, t3008, t2990, t2250, t2989);
        let (t10246, t10249, t10250) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk862::<F>(t10245, t2988, t2775, t607, t2250);
    (t10232, t10233, t10235, t10236, t10237, t10238, t10241, t10242, t10245, t10246, t10249, t10250)
}
