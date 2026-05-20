//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1843/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1843<F: Float>(t15909: F, t12044: F, t12046: F, t12048: F, t12053: F, t12055: F, t12057: F, t12059: F, t12087: F, t20372: F, t20398: F, t9780: F, t9789: F) -> (F, F) {
    let t20520 = F::cast_from(0.32530743900905219526e-1_f64) * t15909;
    let t20521 = -t20372 + t9780 + t20398 + t20520 - t12044 - t12046 - t12048 + t12053 - t12055 - t12057 - t12059 - t9789 + t12087;
    (t20520, t20521)
}
