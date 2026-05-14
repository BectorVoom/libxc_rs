//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1350/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350<F: Float>(t19473: F, t20342: F, t2331: F, t4043: F, t45421: F, t45435: F, t45656: F, t5488: F, t55531: F, t55537: F, t64: F, t656: F, t75592: F, t75601: F, t75613: F, t79748: F, t79755: F, t79812: F) -> (F,) {
    let t79816 = t45421 + 616.0 / 27.0 * t45656 + 44.0 / 3.0 * t55537 - 22.0 / 3.0 * t55531 + 8.0 * t75592 - 8.0 * t75601 + 4.0 / 3.0 * t75613 + 3.0 * t64 * t45435 * t79748 - 9.0 / 2.0 * t64 * t19473 * t5488 + 3.0 / 4.0 * t64 * t2331 * t79755 + t64 * t4043 * t20342 - t64 * t656 * t79812 / 8.0;
    (t79816,)
}
