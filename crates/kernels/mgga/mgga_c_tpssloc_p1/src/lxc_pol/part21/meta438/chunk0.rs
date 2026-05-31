//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1978/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1978<F: Float>(t1216: F, t4733: F, t3578: F, t1653: F, t3494: F, t1090: F, t5012: F, t3490: F, t4993: F, t248: F, t3521: F, t1227: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15469 = t4733 * t1216;
    let t15470 = t3578 * t15469;
    let t15473 = t1653 * t3494;
    let t15474 = t3578 * t15473;
    let t15477 = t5012 * t1090;
    let t15478 = t3578 * t15477;
    let t15484 = t3490 * t4993 / F::cast_from(3456.0_f64);
    let t15486 = t248 * t3521 * t4733;
    let t15488 = t1227 * t15486 / F::cast_from(3456.0_f64);
    (t15469, t15470, t15473, t15474, t15477, t15478, t15484, t15486, t15488)
}
