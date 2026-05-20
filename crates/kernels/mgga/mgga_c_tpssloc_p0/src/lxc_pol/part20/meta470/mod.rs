//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta470<F: Float>(t14731: F, t3440: F, t135: F, t5045: F, t1174: F, t1222: F, t4966: F, t1215: F, t1734: F, t1089: F, t475: F, t607: F) -> (F, F, F, F, F, F) {
        let (t15686, t15691, t15699, t15700, t15701, t15702) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1939::<F>(t14731, t3440, t135, t5045, t1174, t1222, t4966, t1215, t1734, t1089, t475, t607);
    (t15686, t15691, t15699, t15700, t15701, t15702)
}
