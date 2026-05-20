//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta690<F: Float>(t24995: F, t34999: F, t5308: F, t28813: F, t6876: F, t19577: F, t22574: F, t33136: F, t19451: F, t6535: F, t28830: F, t31035: F, t1390: F, t19631: F, t1983: F, t6878: F, t25989: F, t91655: F, t1845: F, t5356: F, t26161: F, t26162: F, t26114: F, t7468: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t96805, t96807, t96813, t96815, t96818) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2135::<F>(t24995, t34999, t5308, t28813, t6876, t19577, t22574, t33136, t19451, t6535, t28830, t31035);
        let (t96827, t96829, t96833, t96837) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2136::<F>(t1390, t19631, t1983, t6878, t25989, t91655, t1845, t5356, t26161, t26162, t26114, t7468);
    (t96805, t96807, t96813, t96815, t96818, t96827, t96829, t96833, t96837)
}
