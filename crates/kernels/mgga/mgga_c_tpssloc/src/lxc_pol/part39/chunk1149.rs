//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1149/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1149<F: Float>(t16123: F, t554: F, t1815: F, t3862: F, t3726: F, t5227: F, t119: F, t16018: F, t210: F, t12308: F, t12310: F, t12317: F, t12323: F, t12325: F, t12330: F, t12336: F, t1315: F, t1363: F, t1369: F, t16321: F, t16325: F, t16331: F, t16333: F, t16338: F, t16341: F, t16346: F, t1831: F, t3783: F, t3876: F, t5240: F, t5314: F, t559: F) -> (F,) {
    let t16347 = t16123 * t554;
    let t16350 = t1815 * t3862;
    let t16354 = 7.0 / 72.0 * t3726 * t5227;
    let t16355 = t119 * t16018;
    let t16356 = t210 * t16355;
    let t16361 = -t16321 * t1369 / 384.0 + t16325 - t12336 * t1831 / 768.0 - t3783 * t5314 / 384.0 + t16331 - t1363 * t16333 / 768.0 + t16338 - t5240 * t3876 / 768.0 - 35.0 / 216.0 * t16341 - 35.0 / 108.0 * t12308 + 7.0 / 144.0 * t12310 - t16346 + t16347 * t559 / 3072.0 + 119.0 / 13824.0 * t16350 - 7.0 / 48.0 * t12317 + t16354 - t1315 * t16356 / 48.0 - 7.0 / 4608.0 * t12323 + 119.0 / 6912.0 * t12325 - t12330;
    (t16361,)
}
