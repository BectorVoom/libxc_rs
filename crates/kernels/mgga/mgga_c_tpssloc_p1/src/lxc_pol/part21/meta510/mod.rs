//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta510<F: Float>(t1569: F, t4433: F, t5762: F, t931: F, t5759: F, t2888: F, t5758: F, t4437: F, t10813: F, t5742: F, t10771: F, t10811: F, t14271: F, t14276: F, t17519: F, t17523: F, t17526: F, t17530: F, t17535: F, t2861: F, t2886: F, t4416: F, t4438: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17538, t17541, t17544, t17547, t17548, t17551, t17554, t17555, t17558) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2156::<F>(t1569, t4433, t5762, t931, t5759, t2888, t5758, t4437, t10813, t5742, t10771, t10811, t14271, t14276, t17519, t17523, t17526, t17530, t17535, t2861, t2886, t4416, t4438);
    (t17538, t17541, t17544, t17547, t17548, t17551, t17554, t17555, t17558)
}
