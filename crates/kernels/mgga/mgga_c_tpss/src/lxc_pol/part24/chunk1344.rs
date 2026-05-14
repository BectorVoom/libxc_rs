//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1344/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1344<F: Float>(t1665: F, t6296: F, t1276: F, t1278: F, t1284: F, t16079: F, t1666: F, t1782: F, t20131: F, t21536: F, t21572: F, t3: F, t4562: F, t5480: F, t550: F, t5761: F, t6280: F, t66149: F, t66155: F, t71022: F, t71025: F, t71030: F, t71071: F, t71115: F) -> (F,) {
    let t71118 = t1665 * t6296;
    let t71120 = t21536 * t1284 + 2.0 * t1666 * t20131 + t5761 * t5480 + t1782 * t16079 + t3 * t71022 * t550 + 2.0 * t71025 + 2.0 * t6280 * t4562 + t1276 * t21572 + t66149 + t71030 + t1278 * (t71071 + t71115) + 2.0 * t71118 + t66155;
    (t71120,)
}
