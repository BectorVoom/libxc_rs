//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3206/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206<F: Float>(t11871: F, t11888: F, t1244: F, t1246: F, t15001: F, t15019: F, t15027: F, t15032: F, t15245: F, t15253: F, t15257: F, t19129: F, t19179: F, t3507: F, t3590: F, t3604: F, t3610: F, t44698: F, t44701: F, t44741: F, t45320: F, t4978: F, t5011: F, t5068: F, t5073: F, t52480: F, t53613: F, t53646: F, t6218: F, t6252: F, t6253: F, t6256: F) -> F {
    let t66737 = -F::cast_from(6.0_f64) * t11888 * t6252 * t44741 - F::cast_from(36.0_f64) * t44698 * t6252 * t44701 * t3507 - F::cast_from(24.0_f64) * t53646 * t52480 * t4978 * t5011 + F::cast_from(12.0_f64) * t53613 * t15001 + F::cast_from(4.0_f64) * t3610 * t6256 * t11871 + F::cast_from(2.0_f64) * t45320 * t6253 - F::cast_from(2.0_f64) * t15245 * t15019 + t1244 * t3590 * t6218 * t1246 - F::cast_from(4.0_f64) * t15245 * t15257 + F::cast_from(4.0_f64) * t15027 * t15253 + F::cast_from(8.0_f64) * t3610 * t19179 * t5068 + F::cast_from(4.0_f64) * t15032 * t5073 + F::cast_from(2.0_f64) * t3604 * t19129;
    t66737
}
