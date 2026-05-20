//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2330/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2330<F: Float>(t1367: F, t19631: F, t820: F, t16336: F, t1831: F, t12308: F, t12325: F, t12330: F, t12335: F, t1363: F, t1369: F, t16321: F, t16346: F, t16350: F, t16354: F, t19904: F, t19915: F, t19917: F, t19921: F, t19926: F, t3778: F, t3783: F, t5240: F, t5310: F, t5314: F, t6422: F, t6427: F, t6431: F) -> (F, F) {
    let t19930 = t1367 * t820 * t19631;
    let t19933 = t16336 * t1831;
    let t19939 = -F::new(35.0) / F::new(216.0) * t12308 - t16346 + F::new(119.0) / F::new(6912.0) * t16350 + t16354 + F::new(119.0) / F::new(13824.0) * t12325 - t12330 - t12335 - t19904 * t1369 / F::new(768.0) + F::new(5.0) / F::new(768.0) * t3783 * t6427 - t3783 * t6431 / F::new(768.0) - t3778 * t6422 / F::new(3072.0) + F::new(5.0) / F::new(384.0) * t5240 * t5310 + F::new(7.0) / F::new(4608.0) * t19915 + F::new(7.0) / F::new(4608.0) * t19917 - F::new(5.0) / F::new(128.0) * t1363 * t19921 + F::new(5.0) / F::new(384.0) * t1363 * t19926 - t1363 * t19930 / F::new(768.0) + F::new(7.0) / F::new(576.0) * t19933 - t16321 * t1831 / F::new(384.0) - t5240 * t5314 / F::new(384.0);
    (t19930, t19939)
}
