//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta733 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta733<F: Float>(t1409: F, t3450: F, t3469: F, t15288: F, t15338: F, t3447: F, t11583: F, t12652: F, t12648: F, t11570: F, t14165: F, t44607: F) -> (F, F, F, F, F, F) {
        let (t52170, t52191, t52216, t52220, t52224, t52228) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2591::<F>(t1409, t3450, t3469, t15288, t15338, t3447, t11583, t12652, t12648, t11570, t14165, t44607);
    (t52170, t52191, t52216, t52220, t52224, t52228)
}
