//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1127/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1127<F: Float>(t136: F, t18517: F, t1113: F, t18241: F, t11211: F, t11487: F, t14766: F, t15347: F, t15348: F, t15349: F, t18494: F, t18497: F, t18500: F, t18503: F, t18505: F, t18508: F, t18510: F, t18512: F, t18515: F) -> (F, F, F) {
    let t18518 = t136 * t18517;
    let t18520 = t1113 * t18241;
    let t18521 = t136 * t18520;
    let t18523 = t11487 - F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t11211 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t14766 - t15347 + t15348 + t15349 - t18494 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t18497 + t18500 / F::cast_from(3.0_f64) + t18503 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18505 - t18508 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18510 + t18512 / F::cast_from(9.0_f64) + t18515 / F::cast_from(18.0_f64) - t18518 / F::cast_from(3.0_f64) - t18521 / F::cast_from(6.0_f64);
    (t18518, t18521, t18523)
}
