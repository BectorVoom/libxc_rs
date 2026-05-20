//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta717<F: Float>(t3070: F, t43198: F, t4578: F, t4574: F, t14192: F, t2960: F, t10510: F, t4641: F, t1020: F, t1616: F, t248: F, t43216: F, t10489: F, t4644: F, t10898: F, t4630: F, t10882: F, t48569: F, t13961: F, t3109: F, t13542: F, t2970: F, t973: F, t13546: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50147, t50169, t50172, t50174, t50181) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558::<F>(t3070, t43198, t4578, t4574, t14192, t2960, t10510, t4641, t1020, t1616, t248, t43216);
        let (t50183, t50189, t50193, t50229, t50242, t50250) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2559::<F>(t10489, t4644, t10898, t4630, t10882, t48569, t13961, t3109, t13542, t2970, t973, t13546);
    (t50147, t50169, t50172, t50174, t50181, t50183, t50189, t50193, t50229, t50242, t50250)
}
