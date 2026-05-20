//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3063/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063<F: Float>(t18834: F, t3315: F, t1117: F, t3313: F, t18258: F, t3307: F, t1147: F, t18710: F, t3400: F, t6063: F, t1157: F, t15121: F, t15133: F, t1695: F, t18899: F, t3396: F, t3404: F, t44300: F, t4835: F, t4858: F, t51366: F, t6056: F, t63563: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F) -> (F, F, F) {
    let t63588 = t18834 * t3315;
    let t63591 = F::cast_from(0.32163958997385070134e2_f64) * t3313 * t63588 * t1117;
    let t63594 = F::cast_from(0.16081979498692535067e2_f64) * t3313 * t18258 * t3307;
    let t63597 = t18710 * t1147;
    let t63602 = t6063 * t3400;
    let t63611 = t63563 + t63566 + t63568 + t63571 + t63574 + t63576 + t63579 + t63582 + t63585 - t63587 - t63591 - t63594 + F::cast_from(0.32163958997385070134e2_f64) * t44300 * t6056 + F::cast_from(0.11696447245269292414e1_f64) * t63597 * t1157 + F::cast_from(0.5848223622634646207e0_f64) * t18899 * t3396 + F::cast_from(0.17315859105681463759e2_f64) * t63602 * t3404 + F::cast_from(0.11696447245269292414e1_f64) * t51366 * t1695 + F::cast_from(0.23392894490538584828e1_f64) * t15121 * t4858 + F::cast_from(0.11696447245269292414e1_f64) * t4835 * t15133;
    (t63591, t63594, t63611)
}
