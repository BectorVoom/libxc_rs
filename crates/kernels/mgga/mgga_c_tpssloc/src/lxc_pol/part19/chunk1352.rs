//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1352/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352<F: Float>(t2331: F, t2332: F, t2358: F, t45421: F, t45422: F, t45424: F, t45426: F, t45428: F, t45430: F, t45432: F, t45435: F, t45436: F, t45444: F, t45505: F, t64: F, t656: F, t9365: F, t9370: F, t9411: F) -> (F,) {
    let t45509 = t45421 + 616.0 / 27.0 * t45422 + 44.0 / 3.0 * t45424 - 22.0 / 3.0 * t45426 + 8.0 * t45428 - 8.0 * t45430 + 4.0 / 3.0 * t45432 + 3.0 * t64 * t45435 * t45436 - 9.0 / 2.0 * t64 * t9365 * t2332 * t2358 + 3.0 / 4.0 * t64 * t2331 * t45444 + t64 * t9370 * t9411 - t64 * t656 * t45505 / 8.0;
    (t45509,)
}
