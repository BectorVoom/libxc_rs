//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 996/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk996<F: Float>(t10022: F, t248: F, t557: F, t555: F, t12238: F, t554: F, t10027: F, t541: F, t12267: F, t1362: F, t3777: F, t3865: F, t1369: F, t1361: F, t2690: F, t1336: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12328 = t10022 * t557 * t248;
    let t12330 = 595.0 / 10368.0 * t555 * t12328;
    let t12331 = t12238 * t554;
    let t12335 = 455.0 / 1296.0 * t10027 * t541;
    let t12336 = t12267 * t1362;
    let t12339 = t3777 * t3865;
    let t12340 = t12339 * t1369;
    let t12344 = t1361 * t2690;
    let t12345 = t1336 * t12344;
    (t12328, t12330, t12331, t12335, t12336, t12339, t12340, t12344, t12345)
}
