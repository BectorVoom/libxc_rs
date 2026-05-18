//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1020/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1020<F: Float>(t33214: F, t7796: F, t28030: F, t8533: F, t128474: F, t128475: F, t128477: F, t128482: F, t128485: F, t128492: F, t128498: F, t128502: F, t128509: F, t1774: F, t2040: F, t28852: F, t28855: F, t31532: F, t33579: F, t5494: F, t652: F, t7042: F, t7670: F, t7801: F, t8329: F, t96686: F) -> F {
    let t128511 = F::new(4.0) * t33214 * t7796;
    let t128513 = F::new(2.0) * t28030 * t8533;
    let t128514 = -F::new(4.0) * t652 * t7670 * t7801 - F::new(2.0) * t1774 * t33579 - F::new(2.0) * t2040 * t96686 - F::new(2.0) * t28852 * t7042 - F::new(4.0) * t28855 * t7042 - F::new(2.0) * t31532 * t5494 + t128474 - t128475 - t128477 - t128482 - t128485 - t128492 - t128498 + t128502 - t128509 - t128511 - t128513 - t8329;
    t128514
}
