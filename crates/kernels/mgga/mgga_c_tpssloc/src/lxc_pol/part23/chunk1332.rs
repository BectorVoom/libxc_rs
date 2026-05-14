//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1332/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1332<F: Float>(t1174: F, t15740: F, t1653: F, t22162: F, t22185: F, t22284: F, t22299: F, t3578: F, t45119: F, t45192: F, t5005: F, t52903: F, t53079: F, t53099: F, t6192: F, t6232: F, t65545: F, t65815: F, t72815: F, t72849: F, t72857: F, t72864: F, t75836: F, t974: F) -> (F,) {
    let t79214 = -19.0 / 288.0 * t65545 * t6232 + 5.0 / 576.0 * t5005 * t22185 + t72815 / 54.0 - t52903 * t22284 / 72.0 + t72849 / 1152.0 - 5.0 / 1944.0 * t72857 - t45119 * t3578 * t22299 * t1653 / 1152.0 + t72864 / 576.0 - t1174 * t974 * t45192 * t75836 / 12.0 - t15740 * t22162 / 384.0 - t65815 * t6192 / 384.0 + t53079 / 2592.0 + t53099 / 2592.0;
    (t79214,)
}
