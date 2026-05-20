//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1154/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1154<F: Float>(t14506: F, t3032: F, t3129: F, t3038: F, t1020: F, t10937: F, t10962: F, t10982: F, t10985: F, t10994: F, t11003: F, t14235: F, t14491: F, t14495: F, t14503: F, t1618: F, t3043: F, t3057: F, t3064: F, t3070: F, t3114: F, t3123: F, t3134: F, t4579: F, t4641: F, t4644: F, t4652: F) -> F {
    let t14507 = t14506 * t3032;
    let t14508 = t14507 * t3129;
    let t14511 = t14507 * t3038;
    let t14523 = F::new(5.0) / F::new(6912.0) * t3070 * t14235 + t1020 * t14491 / F::new(3072.0) + t14495 + t10982 / F::new(864.0) + t10985 / F::new(648.0) - t10994 / F::new(432.0) - t10937 * t4579 / F::new(432.0) + t14503 + t4641 * t3123 / F::new(3072.0) + t14508 * t3134 / F::new(1536.0) - t14511 * t3043 / F::new(3072.0) + t4644 * t3057 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t4644 * t3064 + t10962 * t1618 / F::new(3072.0) + t3114 * t4652 / F::new(1536.0) + t11003 / F::new(2304.0);
    t14523
}
