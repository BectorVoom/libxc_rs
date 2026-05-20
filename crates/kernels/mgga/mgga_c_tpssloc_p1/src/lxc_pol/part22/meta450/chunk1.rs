//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1809/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1809<F: Float>(t19994: F, t3870: F, t820: F, t19744: F, t19871: F, t5248: F, t12369: F, t3805: F, t12346: F, t12366: F, t12429: F, t1363: F, t16233: F, t16394: F, t16400: F, t19940: F, t19942: F, t19945: F, t19951: F, t19958: F, t19962: F, t19966: F, t19972: F, t19976: F, t19981: F, t19986: F, t19991: F, t3803: F, t5246: F, t5259: F, t6396: F) -> (F, F, F, F) {
    let t19996 = t3870 * t820 * t19994;
    let t20000 = t5248 * t19871 * t19744;
    let t20004 = t3805 * t19871 * t12369;
    let t20007 = -F::new(35.0) / F::new(1152.0) * t19940 + F::new(7.0) / F::new(1152.0) * t19942 + t5246 * t19945 / F::new(768.0) + t12429 * t6396 / F::new(384.0) + t3803 * t19951 / F::new(384.0) - F::new(119.0) / F::new(3456.0) * t12346 - F::new(119.0) / F::new(13824.0) * t12366 + t3803 * t19958 / F::new(768.0) - t3803 * t19962 / F::new(3072.0) + t5246 * t19966 / F::new(1536.0) + t16394 * t5259 / F::new(384.0) - t3803 * t19972 / F::new(1536.0) - t3803 * t19976 / F::new(3072.0) - F::new(5.0) / F::new(768.0) * t3803 * t19981 + t3803 * t19986 / F::new(768.0) + t3803 * t19991 / F::new(384.0) + F::new(5.0) / F::new(768.0) * t1363 * t19996 - t16400 - t16233 * t20000 / F::new(512.0) - t5246 * t20004 / F::new(384.0);
    (t19996, t20000, t20004, t20007)
}
