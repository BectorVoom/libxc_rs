//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 772/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk772<F: Float>(t842: F, t9612: F, t2617: F, t2696: F, t849: F, t820: F, t847: F, t9516: F, t2645: F, t2647: F, t9621: F, t2618: F, t2623: F, t2630: F, t2635: F, t2643: F, t2681: F, t2703: F, t843: F, t9967: F, t9974: F, t9978: F, t9983: F, t9986: F, t9988: F) -> (F, F, F) {
    let t9990 = t9612 * t842;
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t9997 = t847 * t820 * t9516;
    let t10003 = t2645 * t9621 * t2647;
    let t10006 = -t2618 * t2681 / F::cast_from(1024.0_f64) + t9967 * t2635 / F::cast_from(512.0_f64) - t9974 * t9978 / F::cast_from(512.0_f64) + t2630 * t9983 / F::cast_from(512.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t9986 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t9988 - t9990 * t849 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t9994 - t843 * t9997 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2623 * t2703 + t2643 * t10003 / F::cast_from(256.0_f64);
    (t9997, t10003, t10006)
}
