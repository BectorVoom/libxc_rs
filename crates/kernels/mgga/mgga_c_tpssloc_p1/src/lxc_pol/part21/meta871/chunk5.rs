//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3205/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205<F: Float>(t11871: F, t11881: F, t11888: F, t1201: F, t15022: F, t15247: F, t15426: F, t1758: F, t18301: F, t19169: F, t19174: F, t19197: F, t3507: F, t3604: F, t3610: F, t3624: F, t3625: F, t44724: F, t44726: F, t44730: F, t470: F, t493: F, t5011: F, t5079: F, t52479: F, t52480: F, t6252: F, t6256: F, t6260: F, t65265: F, t66675: F) -> F {
    let t66702 = F::cast_from(2.0_f64) * t3604 * t19174 - F::cast_from(6.0_f64) * t11888 * t6260 * t15247 + F::cast_from(24.0_f64) * t44724 * t6252 * t44726 * t3507 + F::cast_from(2.0_f64) * t1201 * t19197 + t470 * t493 * t66675 + F::cast_from(24.0_f64) * t52479 * t52480 * t18301 * t5011 + F::cast_from(2.0_f64) * t3610 * t6260 * t11871 - F::cast_from(2.0_f64) * t3624 * t65265 * t3625 + F::cast_from(6.0_f64) * t11881 * t6252 * t44730 - F::cast_from(2.0_f64) * t3624 * t6256 * t15022 - F::cast_from(4.0_f64) * t3624 * t19169 * t5079 - F::cast_from(12.0_f64) * t11888 * t6256 * t15247 + F::cast_from(2.0_f64) * t15426 * t1758;
    t66702
}
