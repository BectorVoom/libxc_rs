//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2333;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta590<F: Float>(t120: F, t6347: F, t1352: F, t3805: F, t5187: F, t550: F, t5249: F, t1307: F, t3870: F, t820: F, t19744: F, t19871: F, t5248: F, t12369: F, t12346: F, t12366: F, t12429: F, t1363: F, t16233: F, t16394: F, t16400: F, t19940: F, t19942: F, t19945: F, t19951: F, t19958: F, t19962: F, t19966: F, t19972: F, t19976: F, t19981: F, t3803: F, t5246: F, t5259: F, t6396: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19984, t19986, t19989, t19991, t19994, t19996, t20000) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2333::<F>(t120, t6347, t1352, t3805, t5187, t550, t5249, t1307, t3870, t820, t19744, t19871, t5248);
        let (t20004, t20007) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2334::<F>(t12369, t19871, t3805, t12346, t12366, t12429, t1363, t16233, t16394, t16400, t19940, t19942, t19945, t19951, t19958, t19962, t19966, t19972, t19976, t19981, t19986, t19991, t19996, t20000, t3803, t5246, t5259, t6396);
    (t19984, t19986, t19989, t19991, t19994, t19996, t20000, t20004, t20007)
}
