//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1230/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1230<F: Float>(t1849: F, t33084: F, t33086: F, t33088: F, t33092: F, t33725: F, t33727: F, t33731: F, t33733: F, t33736: F, t33740: F, t652: F, t7266: F, t7472: F, t8329: F, t8687: F) -> F {
    let t33743 = t1849 * t8687 - F::new(2.0) * t33740 * t652 - F::new(2.0) * t7266 * t7472 + t33084 - F::new(2.0) * t33086 - F::new(2.0) * t33088 - t33092 - t33725 - F::new(2.0) * t33727 - F::new(2.0) * t33731 - F::new(2.0) * t33733 - F::new(2.0) * t33736 - t8329;
    t33743
}
