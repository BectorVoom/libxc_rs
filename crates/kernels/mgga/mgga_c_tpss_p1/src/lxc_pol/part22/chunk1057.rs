//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1057/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1057<F: Float>(t10353: F, t929: F, t926: F, t11493: F, t11497: F, t11501: F, t11508: F, t11509: F, t2685: F, t2731: F, t3924: F, t3935: F, t8577: F, t8588: F, t8954: F, t8966: F, t8972: F, t925: F) -> F {
    let t11512 = t929 * t10353;
    let t11513 = t926 * t11512;
    let t11518 = t8588 / F::cast_from(81.0_f64) - t8954 / F::cast_from(10368.0_f64) - t8966 / F::cast_from(432.0_f64) - t2731 * t11493 / F::cast_from(1536.0_f64) - t2731 * t11497 / F::cast_from(3072.0_f64) + t8577 * t11501 / F::cast_from(3072.0_f64) - t8972 * t3935 / F::cast_from(144.0_f64) + t11508 + t925 * t11509 / F::cast_from(48.0_f64) + t925 * t11513 / F::cast_from(288.0_f64) + t2685 * t3924 / F::cast_from(27.0_f64);
    t11518
}
