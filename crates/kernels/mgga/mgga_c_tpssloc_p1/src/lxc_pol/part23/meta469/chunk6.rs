//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1392/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1392<F: Float>(t10214: F, t1041: F, t13995: F, t14172: F, t1539: F, t1616: F, t21134: F, t21566: F, t21570: F, t21574: F, t21595: F, t2979: F, t3070: F, t3071: F, t43253: F, t43307: F, t4582: F, t50425: F, t62832: F, t70846: F, t70867: F, t70912: F, t70929: F, t76585: F, t76593: F, t76608: F, t76624: F, t77606: F, t973: F, t977: F) -> F {
    let t77761 = t13995 * t21574 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t13995 * t21570 + t3070 * t3071 * t21134 * t1616 / F::cast_from(1152.0_f64) - t43253 - t973 * t2979 * t76593 / F::cast_from(6.0_f64) - t973 * t977 * t76624 / F::cast_from(36.0_f64) + t973 * t2979 * t76608 / F::cast_from(54.0_f64) + t70846 / F::cast_from(576.0_f64) - t70867 / F::cast_from(36.0_f64) - t43307 - t62832 / F::cast_from(162.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1041 * t4582 * t14172 * t77606 + F::cast_from(7.0_f64) / F::cast_from(108.0_f64) * t973 * t10214 * t76585 + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t70912 + F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t50425 + t3070 * t3071 * t21595 * t1539 / F::cast_from(1152.0_f64) + t13995 * t21566 / F::cast_from(384.0_f64) + t70929 / F::cast_from(54.0_f64);
    t77761
}
