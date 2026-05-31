//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1367/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367<F: Float>(t1557: F, t21299: F, t2792: F, t10661: F, t5726: F, t5730: F, t13520: F, t21318: F, t1556: F, t2842: F, t69347: F, t5790: F) -> (F, F, F, F, F) {
    let t77130 = F::cast_from(8.0_f64) * t2792 * t1557 * t21299;
    let t77133 = F::cast_from(0.57895126195293126241e3_f64) * t10661 * t5730 * t5726;
    let t77135 = F::cast_from(0.1929837539843104208e3_f64) * t13520 * t21318;
    let t77138 = F::cast_from(0.64327917994770140268e2_f64) * t2842 * t69347 * t1556;
    let t77139 = t5790 * t5790;
    (t77130, t77133, t77135, t77138, t77139)
}
