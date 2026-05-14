//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1285/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1285<F: Float>(t1268: F, t21011: F, t18547: F, t19580: F, t13955: F, t1760: F, t5754: F, t1689: F, t42710: F, t50656: F, t13565: F, t5522: F, t5532: F, t1338: F, t3490: F, t1321: F, t3537: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68989 = t21011 * t1268;
    let t68992 = 12.0 * t18547 * t19580 * t68989;
    let t69006 = t1760 * t5754 * t13955;
    let t69016 = 2.0 * t42710 * t1689;
    let t69018 = 2.0 * t50656 * t1689;
    let t69020 = 2.0 * t13565 * t5522;
    let t69022 = 2.0 * t13565 * t5532;
    let t69023 = t3490 * t1338;
    let t69025 = 4.0 * t69023 * t1689;
    let t69026 = t1321 * t3537;
    (t68992, t69006, t69016, t69018, t69020, t69022, t69023, t69025, t69026)
}
