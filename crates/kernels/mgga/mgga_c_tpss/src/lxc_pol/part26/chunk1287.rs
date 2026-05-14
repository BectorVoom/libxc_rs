//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1287/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1287<F: Float>(t4674: F, t623: F, t1688: F, t13546: F, t93: F, t21227: F, t5531: F, t19305: F, t6112: F, t19656: F, t19596: F, t6234: F, t2056: F, t21190: F, t4347: F, t1976: F, t4573: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69069 = t623 * t4674;
    let t69071 = 2.0 * t69069 * t1688;
    let t69072 = t93 * t13546;
    let t69074 = 2.0 * t69072 * t1688;
    let t69076 = 2.0 * t21227 * t5531;
    let t69078 = 4.0 * t19305 * t6112;
    let t69080 = 4.0 * t19656 * t6112;
    let t69082 = 4.0 * t6234 * t19596;
    let t69084 = 2.0 * t2056 * t21190;
    let t69086 = 2.0 * t4347 * t21190;
    let t69087 = t1976 * t4573;
    (t69069, t69071, t69074, t69076, t69078, t69080, t69082, t69084, t69086, t69087)
}
