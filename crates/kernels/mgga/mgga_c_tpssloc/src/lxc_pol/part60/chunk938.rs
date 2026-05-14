//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 938/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk938<F: Float>(t128485: F, t128492: F, t128498: F, t128502: F, t128509: F, t128511: F, t128513: F, t128516: F, t128523: F, t128535: F, t128537: F, t128539: F, t128543: F, t128549: F, t28030: F, t29222: F, t29380: F, t34170: F, t7458: F, t8690: F, t8835: F) -> (F,) {
    let t130463 = -2.0 * t28030 * t8835 - t29222 * t8690 + 6.0 * t29380 * t8690 - 4.0 * t34170 * t7458 - t128485 - t128492 - t128498 + t128502 - t128509 - t128511 - t128513 - t128516 - t128523 - t128535 - t128537 - t128539 - t128543 + t128549;
    (t130463,)
}
