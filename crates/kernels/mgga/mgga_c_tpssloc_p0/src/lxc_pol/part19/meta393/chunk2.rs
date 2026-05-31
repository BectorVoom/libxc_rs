//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1493/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493<F: Float>(t2331: F, t2332: F, t2358: F, t45421: F, t45422: F, t45424: F, t45426: F, t45428: F, t45430: F, t45432: F, t45435: F, t45436: F, t45444: F, t45505: F, t64: F, t656: F, t9365: F, t9370: F, t9411: F) -> F {
    let t45509 = t45421 + F::cast_from(616.0_f64) / F::cast_from(27.0_f64) * t45422 + F::cast_from(44.0_f64) / F::cast_from(3.0_f64) * t45424 - F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t45426 + F::cast_from(8.0_f64) * t45428 - F::cast_from(8.0_f64) * t45430 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t45432 + F::cast_from(3.0_f64) * t64 * t45435 * t45436 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t64 * t9365 * t2332 * t2358 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t2331 * t45444 + t64 * t9370 * t9411 - t64 * t656 * t45505 / F::cast_from(8.0_f64);
    t45509
}
