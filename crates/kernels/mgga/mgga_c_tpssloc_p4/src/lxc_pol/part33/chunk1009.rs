//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1009/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1009<F: Float>(t1041: F, t10413: F, t14117: F, t14160: F, t14203: F, t1618: F, t17885: F, t17907: F, t18005: F, t18008: F, t18030: F, t21532: F, t21538: F, t21542: F, t21546: F, t21551: F, t973: F) -> F {
    let t21560 = -t10413 * t21532 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t17885 - t14117 / F::cast_from(4608.0_f64) - t973 * t21538 / F::cast_from(36.0_f64) + t973 * t21542 / F::cast_from(288.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t973 * t21546 - t17907 / F::cast_from(1152.0_f64) - t1041 * t21551 / F::cast_from(768.0_f64) + t18030 * t1618 / F::cast_from(1024.0_f64) - t14160 / F::cast_from(432.0_f64) + t18005 / F::cast_from(1536.0_f64) + t18008 / F::cast_from(1152.0_f64) - t14203 / F::cast_from(6912.0_f64);
    t21560
}
