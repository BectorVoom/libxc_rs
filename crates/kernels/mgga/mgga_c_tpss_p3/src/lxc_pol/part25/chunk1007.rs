//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1007/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1007<F: Float>(t1265: F, t5448: F, t3365: F, t532: F, t5380: F, t1639: F, t1649: F, t5407: F, t1219: F, t5427: F, t10193: F, t1233: F, t1260: F, t13059: F, t13098: F, t13705: F, t13763: F, t13851: F, t13866: F, t1640: F, t220: F, t3374: F, t339: F, t4417: F, t4460: F, t4498: F, t4499: F, t4508: F, t4511: F, t523: F, t5381: F, t5408: F, t5413: F) -> (F, F) {
    let t13888 = t5448 * t1265;
    let t13889 = t3365 * t13888;
    let t13892 = t532 * t5380;
    let t13905 = t1649 * t1639;
    let t13918 = t532 * t5407;
    let t13935 = t1219 * t5427;
    let t13940 = F::cast_from(2.0_f64) * t10193 * t339 * t5381 - t1233 * t13892 * t4508 - F::cast_from(2.0_f64) * t1233 * t13905 * t4508 - t1233 * t13918 * t4508 - t1233 * t13935 * t339 - t1260 * t13851 * t339 - F::cast_from(6.0_f64) * t13059 * t13705 * t13892 - F::cast_from(2.0_f64) * t13098 * t1640 * t339 + F::cast_from(4.0_f64) * t13763 * t4498 * t4499 + t13866 * t220 * t523 + F::cast_from(6.0_f64) * t13892 * t4417 * t4498 + F::cast_from(4.0_f64) * t13905 * t4417 * t4498 + F::cast_from(2.0_f64) * t13918 * t4417 * t4498 - t3374 * t339 * t5408 - t3374 * t339 * t5413 - F::cast_from(2.0_f64) * t339 * t4460 * t4511 - F::cast_from(2.0_f64) * t4460 * t4499 * t4508;
    (t13889, t13940)
}
