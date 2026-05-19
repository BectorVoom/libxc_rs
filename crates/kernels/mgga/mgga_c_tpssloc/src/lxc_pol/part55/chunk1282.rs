//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1282/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1282<F: Float>(t24574: F, t34288: F, t477: F, t8054: F, t32454: F, t7999: F, t1090: F, t118175: F, t1215: F, t1244: F, t1246: F, t1653: F, t1716: F, t2121: F, t2147: F, t24849: F, t27406: F, t27532: F, t27721: F, t32459: F, t32469: F, t32470: F, t34277: F, t34300: F, t3610: F, t462: F, t4930: F, t5068: F, t7283: F, t7327: F, t7362: F, t8082: F, t8891: F) -> F {
    let t125550 = t24574 * t34288;
    let t125558 = t477 * t8054;
    let t125563 = t7999 * t32454;
    let t125568 = F::new(2.0) * t3610 * t34300 * t5068 + F::cast_from(0.16449340668482264365e-1_f64) * t2121 * t462 * t2147 * t27721 - F::cast_from(0.54831135561607547883e-2_f64) * t24849 * t7327 * t8082 * t27532 - F::cast_from(0.43864908449286038307e-1_f64) * t7999 * t32470 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t8891 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t32469 - F::cast_from(0.54831135561607547883e-2_f64) * t125550 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t32459 - F::cast_from(0.54831135561607547883e-2_f64) * t7283 * t7362 * t118175 * t1653 - F::cast_from(0.54831135561607547883e-2_f64) * t7283 * t7362 * t125558 * t1090 - F::cast_from(0.14621636149762012769e-1_f64) * t125563 + t1244 * t34277 * t1215 * t1246;
    t125568
}
