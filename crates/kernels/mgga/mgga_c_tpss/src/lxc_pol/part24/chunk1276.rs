//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1276/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1276<F: Float>(t13955: F, t1760: F, t5754: F, t116: F, t21170: F, t1689: F, t42710: F, t50656: F, t13565: F, t5522: F, t5532: F, t1338: F, t3490: F, t1321: F, t3537: F, t21180: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t69006 = t1760 * t5754 * t13955;
    let t69012 = t21170 * t116;
    let t69016 = 2.0 * t42710 * t1689;
    let t69018 = 2.0 * t50656 * t1689;
    let t69020 = 2.0 * t13565 * t5522;
    let t69022 = 2.0 * t13565 * t5532;
    let t69023 = t3490 * t1338;
    let t69025 = 4.0 * t69023 * t1689;
    let t69026 = t1321 * t3537;
    let t69028 = 4.0 * t69026 * t1689;
    let t69030 = 4.0 * t21180 * t5522;
    (t69006, t69012, t69016, t69018, t69020, t69022, t69023, t69025, t69026, t69028, t69030)
}
