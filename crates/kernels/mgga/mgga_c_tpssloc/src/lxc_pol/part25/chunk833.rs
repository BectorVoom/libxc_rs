//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 833/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk833<F: Float>(t11683: F, t3578: F, t1216: F, t3248: F, t11642: F, t11644: F, t11649: F, t11652: F, t11655: F, t11662: F, t11665: F, t11670: F, t11674: F, t11678: F, t11680: F, t1227: F, t3496: F, t3506: F, t3536: F, t3577: F, t3580: F) -> (F,) {
    let t11684 = t3578 * t11683;
    let t11687 = t3248 * t1216;
    let t11688 = t3578 * t11687;
    let t11691 = t11642 / 1536.0 - t11644 / 4608.0 + t11649 - t11652 / 1536.0 + 5.0 / 2304.0 * t1227 * t11655 + t3536 * t3496 / 1024.0 + t3506 * t11662 / 512.0 - t11665 * t3580 / 768.0 + 5.0 / 4608.0 * t3577 * t11670 - t3577 * t11674 / 1536.0 - t11678 * t11680 / 768.0 - t3577 * t11684 / 1536.0 - t3577 * t11688 / 768.0;
    (t11691,)
}
