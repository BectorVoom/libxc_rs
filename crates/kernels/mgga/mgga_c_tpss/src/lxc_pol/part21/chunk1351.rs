//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1351/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1351<F: Float>(t1656: F, t3326: F, t520: F, t18497: F, t1232: F, t1265: F, t3260: F, t13051: F, t1639: F, t18471: F, t18474: F, t18490: F, t18496: F, t18499: F, t18504: F, t18508: F, t19497: F, t19500: F, t19507: F, t19509: F, t19526: F, t19530: F, t19536: F, t19541: F, t3366: F, t3385: F, t4459: F, t4517: F, t5731: F, t5734: F, t5739: F, t5745: F, t5748: F, t5751: F, t60649: F, t60653: F, t60778: F, t6255: F, t6263: F, t6268: F, t65667: F) -> (F,) {
    let t65783 = t1656 * t3326 * t520;
    let t65788 = t18497 * t1656;
    let t65818 = t3260 * t1265 * t1232;
    let t65843 = -4.0 * t60649 * t19536 - 2.0 * t18496 * t18497 * t65783 + t60778 * t6268 + 12.0 * t60653 * t65788 * t18499 - 2.0 * t18474 * t4517 + 2.0 * t5739 * t5745 * t19497 * t1232 * t520 + 2.0 * t60778 * t6263 - 2.0 * t19507 * t5751 + t5739 * t5745 * t18471 * t1639 * t520 + 2.0 * t5739 * t5745 * t5731 * t4459 * t520 - 4.0 * t18496 * t18497 * t1639 * t18499 + 8.0 * t18496 * t19541 * t1639 * t65818 + 4.0 * t19509 * t18504 + 2.0 * t65667 * t5748 - 6.0 * t5739 * t18490 * t6255 * t3366 - t19500 * t3385 + 4.0 * t5734 * t13051 + 2.0 * t19509 * t18508 - 12.0 * t5739 * t18490 * t19530 * t1265 - 12.0 * t5739 * t18490 * t19526 * t1265;
    (t65843,)
}
