//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2370/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370<F: Float>(t2022: F, t5381: F, t26509: F, t580: F, t1404: F, t7758: F, t1395: F, t7774: F, t1396: F, t1398: F, t26510: F, t26555: F, t3: F, t3932: F, t5364: F, t7020: F, t80599: F, t80601: F, t80605: F, t86640: F, t91792: F, t91806: F) -> F {
    let t91813 = F::cast_from(2.0_f64) * t2022 * t5381;
    let t91816 = F::cast_from(2.0_f64) * t26509 * t580;
    let t91818 = F::cast_from(2.0_f64) * t7758 * t1404;
    let t91824 = F::cast_from(2.0_f64) * t1395 * t7774;
    let t91827 = t1398 * (t86640 + t91806) + t80605 + F::cast_from(2.0_f64) * t1396 * t26555 + t3932 * t7774 + t91813 + F::cast_from(2.0_f64) * t80599 + t91816 + t91818 + F::cast_from(2.0_f64) * t5364 * t7020 + F::cast_from(2.0_f64) * t26510 * t1404 + t91824 + t3 * t91792 * t580 + t80601;
    t91827
}
