//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2253/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2253<F: Float>(t11369: F, t14722: F, t14766: F, t14768: F, t14782: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18229: F, t18234: F, t18243: F, t18494: F, t18505: F, t18512: F, t18521: F, t18731: F, t18759: F, t18762: F, t18783: F) -> F {
    let t18785 = F::new(0.258925e1) * t18731 - t11369 - F::new(0.5519e-1) * t18512 + F::new(0.82785e-1) * t18521 + F::cast_from(0.67094444444444444443e-1_f64) * t18203 - F::cast_from(0.20128333333333333333e0_f64) * t18219 - F::cast_from(0.10064166666666666667e0_f64) * t18229 + F::new(0.301925e0) * t18243 + F::cast_from(0.18396666666666666667e-1_f64) * t18494 - F::new(0.11038e0) * t18505 + t18759 - F::new(0.1294625e1) * t18762 + F::cast_from(0.18396666666666666667e0_f64) * t14766 + t14768 - F::cast_from(0.40256666666666666668e0_f64) * t14722 - t14782 - F::cast_from(0.20128333333333333333e0_f64) * t18234 + F::cast_from(0.33547222222222222222e0_f64) * t18208 - F::new(0.12077e1) * t18213 - F::cast_from(0.40256666666666666666e0_f64) * t18217 + F::new(0.181155e1) * t18223 + t18783;
    t18785
}
