//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta523<F: Float>(t17157: F, t4510: F, t17161: F, t13798: F, t17152: F, t10236: F, t5392: F, t10235: F, t13851: F, t4514: F, t10287: F, t10333: F, t10339: F, t13893: F, t13896: F, t13907: F, t13909: F, t13915: F, t2986: F) -> (F, F, F, F, F, F, F) {
        let (t17854, t17857, t17860, t17863, t17864, t17867, t17873) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2176::<F>(t17157, t4510, t17161, t13798, t17152, t10236, t5392, t10235, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t2986);
    (t17854, t17857, t17860, t17863, t17864, t17867, t17873)
}
