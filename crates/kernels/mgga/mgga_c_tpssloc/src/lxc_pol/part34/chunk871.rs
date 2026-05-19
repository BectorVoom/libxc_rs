//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 871/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk871<F: Float>(t20857: F, t819: F, t820: F, t20800: F, t847: F, t20756: F, t210: F, t214: F, t221: F, t4128: F, t5544: F, t12986: F, t13010: F, t13022: F, t16769: F, t16784: F, t16792: F, t16794: F, t4127: F, t787: F, t9540: F, t9559: F, t9572: F, t9579: F, t9583: F) -> (F, F, F) {
    let t20904 = t819 * t820 * t20857;
    let t20908 = t847 * t820 * t20800;
    let t20923 = t210 * t214 * t20756;
    let t20927 = t221 * t4128 * t5544;
    let t20933 = t210 * t214 * t20800;
    let t20936 = -t9540 + F::cast_from(0.49999999999999999998e-2_f64) * t12986 - t9572 - F::cast_from(0.34999999999999999998e-1_f64) * t16769 - F::cast_from(0.38888888888888888888e-1_f64) * t13010 - F::cast_from(0.74999999999999999997e-2_f64) * t16784 + F::cast_from(0.24999999999999999999e-2_f64) * t16792 - F::cast_from(0.19999999999999999999e-1_f64) * t9559 * t20923 + F::cast_from(0.14999999999999999999e-1_f64) * t4127 * t20927 + t9579 + F::cast_from(0.11666666666666666666e-1_f64) * t16794 - F::cast_from(0.15833333333333333333e-1_f64) * t13022 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t20933 - t9583;
    (t20904, t20908, t20936)
}
