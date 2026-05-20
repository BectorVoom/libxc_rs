//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3060/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3060<F: Float>(t11285: F, t6084: F, t18785: F, t3403: F, t3307: F, t3313: F, t5989: F, t11275: F, t3265: F, t6024: F, t11310: F, t11361: F, t1155: F, t1156: F, t14829: F, t15126: F, t15136: F, t15182: F, t15185: F, t15210: F, t15222: F, t15226: F, t18615: F, t18619: F, t18622: F, t18623: F, t18643: F, t3351: F, t3357: F, t3376: F, t3377: F, t3395: F, t3401: F, t43692: F, t44220: F, t44223: F, t4861: F, t51376: F, t51680: F, t6068: F, t6069: F, t6088: F, t63283: F) -> (F, F, F) {
    let t63519 = t6084 * t11285;
    let t63533 = t18785 * t3403;
    let t63557 = F::new(6.0) * t3313 * t5989 * t3307;
    let t63560 = F::cast_from(0.57895126195293126241e3_f64) * t11275 * t6024 * t3265;
    let t63561 = -F::cast_from(0.23392894490538584828e1_f64) * t3376 * t63283 * t1156 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t18643 * t3351 + F::cast_from(0.70178683471615754484e1_f64) * t15126 * t15210 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t18615 * t3395 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t63519 * t3377 + F::cast_from(0.34631718211362927518e2_f64) * t3401 * t4861 * t14829 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t18622 * t3395 + F::cast_from(0.91082604192152556044e5_f64) * t44223 * t6068 * t43692 * t3377 + F::cast_from(0.34631718211362927518e2_f64) * t3401 * t63533 * t1155 + F::cast_from(0.69263436422725855036e2_f64) * t11361 * t18619 + F::cast_from(0.20508037716432813316e4_f64) * t44220 * t18623 - F::cast_from(0.23392894490538584828e1_f64) * t15136 * t15182 - F::cast_from(0.2077903092681775651e3_f64) * t51680 * t15185 + F::cast_from(0.34631718211362927517e2_f64) * t15126 * t15222 + F::cast_from(0.20508037716432813315e4_f64) * t51376 * t15226 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t6069 * t3395 + F::cast_from(0.6233709278045326953e3_f64) * t11310 * t6088 * t3377 - t63557 - t63560;
    (t63557, t63560, t63561)
}
