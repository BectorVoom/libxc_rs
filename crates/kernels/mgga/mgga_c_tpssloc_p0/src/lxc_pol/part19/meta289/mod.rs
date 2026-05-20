//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1058;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1059;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta289<F: Float>(t12328: F, t555: F, t12238: F, t554: F, t10027: F, t541: F, t12267: F, t1362: F, t3777: F, t3865: F, t1369: F, t1361: F, t2690: F, t1336: F, t12215: F, t12317: F, t12320: F, t12323: F, t12325: F, t3783: F, t3876: F, t559: F, t241: F, t67: F, t6924: F, t12156: F, t820: F, t3866: F, t3872: F, t12012: F, t1367: F, t1339: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12330, t12331, t12335, t12336, t12339, t12340, t12344) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1058::<F>(t12328, t555, t12238, t554, t10027, t541, t12267, t1362, t3777, t3865, t1369, t1361, t2690);
        let (t12345, t12348) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1059::<F>(t12344, t1336, t1369, t12215, t12317, t12320, t12323, t12325, t12330, t12331, t12335, t12336, t12340, t3783, t3876, t559);
        let (t12351, t12353, t12356, t12358, t12361, t12364) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1060::<F>(t241, t67, t6924, t12156, t820, t3866, t3872, t3876, t12012, t1367, t1339, t2690);
    (t12331, t12336, t12339, t12344, t12345, t12348, t12351, t12353, t12356, t12358, t12361, t12364)
}
