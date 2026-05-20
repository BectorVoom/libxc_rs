//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta457<F: Float>(t15908: F, t2375: F, t12045: F, t12050: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F, t12089: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15904: F, t3734: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t9789: F, t9793: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15909, t15910, t15911, t15915, t15916, t15917, t15921, t15923, t15927, t15928, t15929) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2017::<F>(t15908, t2375, t12045, t12050, t12052, t12054, t5151, t750, t17, t12089, t12091, t12044, t12048, t12057, t12059, t12087, t12094, t15904, t3734, t3918, t3919, t5122, t5126, t5161, t5187, t5308, t9789, t9793);
    (t15909, t15910, t15911, t15915, t15916, t15917, t15921, t15923, t15927, t15928, t15929)
}
