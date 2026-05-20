//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1518;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1519;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta415<F: Float>(t1352: F, t19956: F, t5248: F, t5250: F, t5249: F, t5287: F, t19871: F, t120: F, t6330: F, t12419: F, t6347: F, t3805: F, t5187: F, t550: F, t1307: F, t3870: F, t820: F, t19744: F, t12369: F, t12346: F, t12366: F, t12429: F, t1363: F, t16233: F, t16394: F, t16400: F, t19940: F, t19942: F, t19945: F, t19951: F, t19958: F, t3803: F, t5246: F, t5259: F, t6396: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19962, t19966, t19972, t19976, t19981, t19986) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1518::<F>(t1352, t19956, t5248, t5250, t5249, t5287, t19871, t120, t6330, t12419, t6347, t3805);
        let (t19991, t19994, t19996, t20000, t20004) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1519::<F>(t5187, t550, t3805, t5249, t1307, t6347, t3870, t820, t19744, t19871, t5248, t12369);
        let t20007 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1520::<F>(t12346, t12366, t12429, t1363, t16233, t16394, t16400, t19940, t19942, t19945, t19951, t19958, t19962, t19966, t19972, t19976, t19981, t19986, t19991, t19996, t20000, t20004, t3803, t5246, t5259, t6396);
    (t19962, t19966, t19972, t19976, t19981, t19986, t19991, t19994, t19996, t20000, t20004, t20007)
}
