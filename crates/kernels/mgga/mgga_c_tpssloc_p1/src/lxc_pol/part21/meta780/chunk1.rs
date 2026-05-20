//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2707/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707<F: Float>(t1827: F, t54532: F, t16232: F, t5234: F, t12419: F, t12429: F, t1363: F, t16208: F, t16226: F, t16235: F, t16278: F, t16312: F, t19855: F, t19871: F, t19962: F, t20468: F, t3719: F, t3734: F, t3795: F, t3853: F, t3870: F, t39978: F, t40065: F, t40070: F, t40079: F, t5235: F, t5246: F, t5289: F, t5334: F, t5344: F, t54178: F, t57033: F, t57041: F, t57044: F, t57046: F, t57057: F, t57071: F, t6330: F, t6347: F, t820: F) -> F {
    let t57073 = t54532 * t1827;
    let t57081 = t5234 * t16232;
    let t57084 = -F::new(119.0) / F::new(3456.0) * t40065 + F::new(595.0) / F::new(3456.0) * t40079 + t57033 * t3795 / F::new(1536.0) + F::new(35.0) / F::new(128.0) * t1363 * t40070 * t820 * t6330 * t3734 + F::new(119.0) / F::new(6912.0) * t57041 + F::new(5.0) / F::new(96.0) * t5334 * t57044 * t57046 * t20468 * t16312 + F::new(5.0) / F::new(768.0) * t1363 * t3870 * t820 * t6347 * t3719 + F::new(7.0) / F::new(2304.0) * t57057 - t19855 * t3853 / F::new(3072.0) - t54178 * t1827 / F::new(1536.0) - t16278 * t5289 / F::new(768.0) - t5235 * t16208 / F::new(1536.0) - F::new(5.0) / F::new(192.0) * t5344 * t57044 * t57046 * t16226 - F::new(7.0) / F::new(1152.0) * t57071 - F::new(119.0) / F::new(6912.0) * t57073 + F::new(5.0) / F::new(384.0) * t5246 * t12419 * t19871 * t39978 - t12429 * t19962 / F::new(1536.0) - t57081 * t16235 / F::new(256.0);
    t57084
}
