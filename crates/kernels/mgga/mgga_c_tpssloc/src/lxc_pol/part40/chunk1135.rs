//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1135/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1135<F: Float>(t12308: F, t12325: F, t12330: F, t12335: F, t1363: F, t1369: F, t16321: F, t16346: F, t16350: F, t16354: F, t1831: F, t19904: F, t19915: F, t19917: F, t19921: F, t19926: F, t19930: F, t19933: F, t3778: F, t3783: F, t5240: F, t5310: F, t5314: F, t6422: F, t6427: F, t6431: F) -> (F,) {
    let t19939 = -35.0 / 216.0 * t12308 - t16346 + 119.0 / 6912.0 * t16350 + t16354 + 119.0 / 13824.0 * t12325 - t12330 - t12335 - t19904 * t1369 / 768.0 + 5.0 / 768.0 * t3783 * t6427 - t3783 * t6431 / 768.0 - t3778 * t6422 / 3072.0 + 5.0 / 384.0 * t5240 * t5310 + 7.0 / 4608.0 * t19915 + 7.0 / 4608.0 * t19917 - 5.0 / 128.0 * t1363 * t19921 + 5.0 / 384.0 * t1363 * t19926 - t1363 * t19930 / 768.0 + 7.0 / 576.0 * t19933 - t16321 * t1831 / 384.0 - t5240 * t5314 / 384.0;
    (t19939,)
}
