//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1465/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1465<F: Float>(t13222: F, t16968: F, t16673: F, t842: F, t13345: F, t13365: F, t1516: F, t16914: F, t16918: F, t16924: F, t16928: F, t16932: F, t16937: F, t16940: F, t16942: F, t16946: F, t16951: F, t16954: F, t16957: F, t16961: F, t16965: F, t2571: F, t2643: F, t4172: F, t4178: F, t4261: F, t5593: F, t843: F, t849: F, t9559: F, t9642: F) -> (F, F) {
    let t16969 = t13222 * t16968;
    let t16976 = t16673 * t842;
    let t16979 = t2643 * t16914 / F::cast_from(384.0_f64) + t2643 * t16918 / F::cast_from(768.0_f64) + t9642 * t5593 / F::cast_from(384.0_f64) + t2643 * t16924 / F::cast_from(384.0_f64) - t4178 * t16928 / F::cast_from(192.0_f64) + t13345 - t4178 * t16932 / F::cast_from(384.0_f64) + t4178 * t16937 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t16940 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t16942 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t843 * t16946 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t843 * t16951 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t16954 - t9559 * t16957 / F::cast_from(4.0_f64) + t2571 * t16961 / F::cast_from(8.0_f64) + t2571 * t16965 / F::cast_from(16.0_f64) + t2643 * t16969 / F::cast_from(384.0_f64) - t13365 * t1516 / F::cast_from(384.0_f64) - t4172 * t4261 / F::cast_from(384.0_f64) - t16976 * t849 / F::cast_from(768.0_f64);
    (t16969, t16979)
}
