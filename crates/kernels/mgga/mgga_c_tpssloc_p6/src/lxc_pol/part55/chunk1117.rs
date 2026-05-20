//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1117/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1117<F: Float>(t1458: F, t8913: F, t1442: F, t32666: F, t32674: F, t32676: F, t32679: F, t32684: F, t32784: F, t33084: F, t33688: F, t33691: F, t33693: F, t33697: F, t33725: F, t33727: F, t33731: F, t33733: F, t652: F, t7266: F, t7989: F, t8329: F) -> (F, F) {
    let t34203 = t8913 * t1458;
    let t34210 = -t1442 * t8913 - F::new(2.0) * t34203 * t652 - F::new(4.0) * t7266 * t7989 - t32666 - t32674 - t32676 - t32679 + t32684 + t32784 + t33084 - F::new(4.0) * t33688 - F::new(4.0) * t33691 - F::new(4.0) * t33693 - F::new(4.0) * t33697 - F::new(2.0) * t33725 - F::new(4.0) * t33727 - F::new(4.0) * t33731 - F::new(4.0) * t33733 - t8329;
    (t34203, t34210)
}
