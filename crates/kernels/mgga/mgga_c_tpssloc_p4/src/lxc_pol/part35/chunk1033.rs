//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1033/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1033<F: Float>(t1041: F, t10413: F, t14117: F, t14160: F, t14203: F, t1618: F, t17885: F, t17907: F, t18005: F, t18008: F, t18030: F, t21532: F, t21538: F, t21542: F, t21546: F, t21551: F, t973: F) -> F {
    let t21560 = -t10413 * t21532 / F::new(1536.0) + F::new(5.0) / F::new(6912.0) * t17885 - t14117 / F::new(4608.0) - t973 * t21538 / F::new(36.0) + t973 * t21542 / F::new(288.0) + F::new(7.0) / F::new(648.0) * t973 * t21546 - t17907 / F::new(1152.0) - t1041 * t21551 / F::new(768.0) + t18030 * t1618 / F::new(1024.0) - t14160 / F::new(432.0) + t18005 / F::new(1536.0) + t18008 / F::new(1152.0) - t14203 / F::new(6912.0);
    t21560
}
