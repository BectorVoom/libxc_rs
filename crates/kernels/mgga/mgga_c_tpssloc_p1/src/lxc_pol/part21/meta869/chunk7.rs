//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3189/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189<F: Float>(t11719: F, t11728: F, t11738: F, t15545: F, t15620: F, t15625: F, t15656: F, t18303: F, t19056: F, t3248: F, t3506: F, t3509: F, t3516: F, t3577: F, t3578: F, t44896: F, t44968: F, t44972: F, t44976: F, t4582: F, t5024: F, t52991: F, t52993: F, t52999: F, t53001: F, t6219: F) -> F {
    let t66219 = -t52991 / F::cast_from(972.0_f64) - t52993 / F::cast_from(3456.0_f64) + t52999 / F::cast_from(648.0_f64) + t44896 * t18303 / F::cast_from(256.0_f64) + t53001 / F::cast_from(576.0_f64) + t44968 / F::cast_from(10368.0_f64) + t44972 / F::cast_from(20736.0_f64) + t44976 / F::cast_from(10368.0_f64) + t3506 * t4582 * t19056 * t15620 / F::cast_from(1536.0_f64) + t11719 * t4582 * t19056 * t15625 / F::cast_from(512.0_f64) - t11728 * t4582 * t19056 * t3509 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t5024 * t15545 - F::cast_from(5.0_f64) / F::cast_from(216.0_f64) * t5024 * t15656 - t3577 * t3578 * t6219 * t3248 / F::cast_from(2304.0_f64) + t11738 * t4582 * t19056 * t3516 / F::cast_from(3072.0_f64);
    t66219
}
