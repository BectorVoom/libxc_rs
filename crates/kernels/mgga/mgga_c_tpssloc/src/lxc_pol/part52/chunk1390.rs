//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1390/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1390<F: Float>(t2165: F, t26135: F, t652: F, t120735: F, t120738: F, t120740: F, t120742: F, t120744: F, t120747: F, t120749: F, t120751: F, t120753: F, t1393: F, t1849: F, t22461: F, t31892: F, t33720: F, t7989: F) -> F {
    let t123244 = t652 * t2165 * t26135;
    let t123257 = t1393 * t33720 + t1849 * t31892 - F::new(2.0) * t22461 * t7989 - t120735 - t120738 - F::new(2.0) * t120740 - F::new(2.0) * t120742 - F::new(2.0) * t120744 - F::new(2.0) * t120747 - F::new(2.0) * t120749 - F::new(2.0) * t120751 - F::new(2.0) * t120753 - F::new(2.0) * t123244;
    t123257
}
