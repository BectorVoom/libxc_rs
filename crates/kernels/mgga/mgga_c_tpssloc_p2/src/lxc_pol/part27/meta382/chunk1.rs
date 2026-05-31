//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1573/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1573<F: Float>(t14506: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t14491: F, t14495: F, t14503: F, t1618: F, t3043: F, t3057: F, t3064: F, t3070: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4644: F, t4652: F) -> (F, F) {
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14523 = F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3070 * t14235 + t1020 * t14491 / F::cast_from(3072.0_f64) + t14495 + t10982 / F::cast_from(864.0_f64) + t10985 / F::cast_from(648.0_f64) - t10994 / F::cast_from(432.0_f64) - t10937 * t4579 / F::cast_from(432.0_f64) + t14503 + t4641 * t3123 / F::cast_from(3072.0_f64) + t14508 * t3134 / F::cast_from(1536.0_f64) - t14511 * t3043 / F::cast_from(3072.0_f64) + t4644 * t3057 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t4644 * t3064 + t10962 * t1618 / F::cast_from(3072.0_f64) + t3114 * t4652 / F::cast_from(1536.0_f64) + t11003 / F::cast_from(2304.0_f64);
    (t14507, t14523)
}
