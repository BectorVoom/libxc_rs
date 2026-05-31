//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1241/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1241<F: Float>(t16355: F, t210: F, t12308: F, t12310: F, t12317: F, t12323: F, t12325: F, t12330: F, t12336: F, t1315: F, t1363: F, t1369: F, t16321: F, t16325: F, t16331: F, t16333: F, t16338: F, t16341: F, t16346: F, t16347: F, t16350: F, t16354: F, t1831: F, t3783: F, t3876: F, t5240: F, t5314: F, t559: F) -> F {
    let t16356 = t210 * t16355;
    let t16361 = -t16321 * t1369 / F::cast_from(384.0_f64) + t16325 - t12336 * t1831 / F::cast_from(768.0_f64) - t3783 * t5314 / F::cast_from(384.0_f64) + t16331 - t1363 * t16333 / F::cast_from(768.0_f64) + t16338 - t5240 * t3876 / F::cast_from(768.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t16341 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t12308 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t12310 - t16346 + t16347 * t559 / F::cast_from(3072.0_f64) + F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t16350 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t12317 + t16354 - t1315 * t16356 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t12323 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t12325 - t12330;
    t16361
}
