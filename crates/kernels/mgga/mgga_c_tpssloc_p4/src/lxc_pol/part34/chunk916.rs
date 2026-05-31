//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 916/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk916<F: Float>(t21525: F, t3071: F, t10403: F, t1041: F, t13966: F, t13995: F, t17621: F, t17625: F, t17656: F, t17660: F, t17662: F, t17668: F, t21503: F, t21512: F, t21516: F, t21520: F, t3039: F, t3070: F, t5909: F) -> F {
    let t21526 = t3071 * t21525;
    let t21529 = t17621 / F::cast_from(216.0_f64) - t13966 / F::cast_from(4608.0_f64) - t17625 / F::cast_from(144.0_f64) - t3039 * t21503 / F::cast_from(1024.0_f64) - t17656 / F::cast_from(1536.0_f64) + t17660 / F::cast_from(2304.0_f64) + t17662 / F::cast_from(768.0_f64) + t17668 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1041 * t21512 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1041 * t21516 - t3070 * t21520 / F::cast_from(768.0_f64) + t13995 * t5909 / F::cast_from(768.0_f64) + t10403 * t21526 / F::cast_from(768.0_f64);
    t21529
}
