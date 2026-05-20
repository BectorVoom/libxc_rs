//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3058/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058<F: Float>(t3265: F, t3313: F, t6021: F, t11190: F, t5989: F, t14850: F, t14937: F, t3375: F, t6063: F, t1136: F, t11365: F, t14829: F, t15153: F, t15165: F, t15179: F, t15219: F, t1695: F, t18615: F, t18622: F, t3376: F, t3377: F, t3378: F, t3395: F, t3401: F, t3403: F, t436: F, t44155: F, t51382: F, t51389: F, t51392: F, t51486: F, t51521: F, t51727: F, t6085: F, t6088: F, t63280: F, t63283: F, t63290: F, t63325: F, t63346: F, t63376: F, t63424: F) -> (F, F, F, F) {
    let t63446 = F::new(6.0) * t3313 * t6021 * t3265;
    let t63449 = F::new(24.0) * t11190 * t5989 * t3265;
    let t63451 = F::new(12.0) * t14850 * t14937;
    let t63454 = t6063 * t3375;
    let t63457 = F::cast_from(0.8276162067083744048e4_f64) * t51486 * t51521 * t1136 + F::new(24.0) * t51382 * t15153 - F::cast_from(0.4155806185363551302e3_f64) * t51727 * t15219 - t63280 + F::cast_from(0.14035736694323150897e2_f64) * t51389 * t15179 + F::cast_from(0.34631718211362927518e2_f64) * t3401 * t63283 * t3403 + t63290 - F::new(0.310907e-1) * (t63325 + t63346 + t63376 + t63424) * t436 - F::cast_from(0.23392894490538584828e1_f64) * t3376 * t1695 * t14829 - F::cast_from(0.10389515463408878255e3_f64) * t11365 * t6088 * t3395 - F::cast_from(0.12304822629859687989e5_f64) * t44155 * t18622 * t3377 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t6085 * t3395 - F::cast_from(0.10389515463408878255e3_f64) * t11365 * t18615 * t3377 - t63446 + t63449 - t63451 - F::cast_from(0.77193501593724168323e3_f64) * t51392 * t15165 - F::cast_from(0.11696447245269292414e1_f64) * t63454 * t3378;
    (t63446, t63449, t63451, t63457)
}
