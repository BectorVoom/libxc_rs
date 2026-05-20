//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2076;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta569<F: Float>(t1032: F, t10375: F, t370: F, t374: F, t376: F, t9697: F, t10473: F, t361: F, t363: F, t42342: F, t42345: F, t3131: F, t221: F, t339: F, t42813: F, t10216: F, t2978: F, t10479: F, t42333: F, t3061: F, t676: F, t11065: F, t42387: F, t1005: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43248, t43253, t43288, t43291, t43292) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2076::<F>(t1032, t10375, t370, t374, t376, t9697, t10473, t361, t363, t42342, t42345, t3131);
        let (t43307, t43317, t43322, t43338, t43361, t43382) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077::<F>(t221, t339, t42813, t10216, t2978, t10479, t42333, t3061, t676, t11065, t42387, t1005, t10375);
    (t43248, t43253, t43288, t43291, t43292, t43307, t43317, t43322, t43338, t43361, t43382)
}
