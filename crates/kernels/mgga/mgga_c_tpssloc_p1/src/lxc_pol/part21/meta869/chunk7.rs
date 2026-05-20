//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3189/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3189<F: Float>(t11719: F, t11728: F, t11738: F, t15545: F, t15620: F, t15625: F, t15656: F, t18303: F, t19056: F, t3248: F, t3506: F, t3509: F, t3516: F, t3577: F, t3578: F, t44896: F, t44968: F, t44972: F, t44976: F, t4582: F, t5024: F, t52991: F, t52993: F, t52999: F, t53001: F, t6219: F) -> F {
    let t66219 = -t52991 / F::new(972.0) - t52993 / F::new(3456.0) + t52999 / F::new(648.0) + t44896 * t18303 / F::new(256.0) + t53001 / F::new(576.0) + t44968 / F::new(10368.0) + t44972 / F::new(20736.0) + t44976 / F::new(10368.0) + t3506 * t4582 * t19056 * t15620 / F::new(1536.0) + t11719 * t4582 * t19056 * t15625 / F::new(512.0) - t11728 * t4582 * t19056 * t3509 / F::new(512.0) - F::new(5.0) / F::new(1296.0) * t5024 * t15545 - F::new(5.0) / F::new(216.0) * t5024 * t15656 - t3577 * t3578 * t6219 * t3248 / F::new(2304.0) + t11738 * t4582 * t19056 * t3516 / F::new(3072.0);
    t66219
}
