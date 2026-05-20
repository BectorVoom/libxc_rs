//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2658/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658<F: Float>(t20595: F, t68: F, t1340: F, t20556: F, t3799: F, t20570: F, t1362: F, t1354: F, t1369: F, t16278: F, t16321: F, t16394: F, t1831: F, t19868: F, t19904: F, t19930: F, t19991: F, t20479: F, t20492: F, t3783: F, t39936: F, t40035: F, t5235: F, t5240: F, t5314: F, t57024: F, t6417: F, t6431: F) -> (F, F) {
    let t74289 = t20595 * t68;
    let t74290 = t74289 * t1340;
    let t74297 = t3799 * t20556;
    let t74299 = t3799 * t20570;
    let t74311 = t74289 * t1362;
    let t74316 = t16394 * t19991 / F::new(128.0) + t39936 - t74290 * t1354 / F::new(3072.0) - t16278 * t6417 / F::new(1024.0) - t5235 * t19868 / F::new(1024.0) + F::new(7.0) / F::new(4608.0) * t74297 + F::new(7.0) / F::new(4608.0) * t74299 - t40035 * t20492 / F::new(512.0) - t16321 * t6431 / F::new(256.0) - t5240 * t19930 / F::new(256.0) - t57024 * t1831 / F::new(256.0) - t19904 * t5314 / F::new(256.0) - t74311 * t1369 / F::new(768.0) - t3783 * t20479 / F::new(768.0);
    (t74289, t74316)
}
