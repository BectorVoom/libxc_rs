//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1051/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1051<F: Float>(t117006: F, t117084: F, t122698: F, t124476: F, t124531: F, t124540: F, t1458: F, t1849: F, t19577: F, t2040: F, t2075: F, t22574: F, t2314: F, t23938: F, t25988: F, t26161: F, t26163: F, t26875: F, t26977: F, t27170: F, t27219: F, t27226: F, t32108: F, t32278: F, t33363: F, t33883: F, t4034: F, t5361: F, t574: F, t652: F, t7042: F, t7156: F, t7171: F, t7787: F, t7802: F, t8780: F, t9003: F, t92090: F) -> F {
    let t124552 = -F::cast_from(4.0_f64) * t92090 * t2040 + F::cast_from(2.0_f64) * t26161 * t124476 * t26163 + F::cast_from(12.0_f64) * t122698 * t26875 - F::cast_from(3.0_f64) * t22574 * t117084 * t19577 + t32278 * t1849 + t8780 * t5361 - F::cast_from(2.0_f64) * t2314 * t33883 - F::cast_from(2.0_f64) * t4034 * t33883 - F::cast_from(2.0_f64) * t652 * t32108 * t1458 - F::cast_from(4.0_f64) * t652 * t2075 * t27170 - F::cast_from(4.0_f64) * t23938 * t7802 - F::cast_from(4.0_f64) * t26977 * t7802 - F::cast_from(4.0_f64) * t7042 * t27226 + (t124531 + t124540) * t574 - F::cast_from(2.0_f64) * t7787 * t7156 + F::cast_from(6.0_f64) * t33363 * t7171 - F::cast_from(4.0_f64) * t9003 * t27219 + F::cast_from(6.0_f64) * t22574 * t117006 * t25988;
    t124552
}
