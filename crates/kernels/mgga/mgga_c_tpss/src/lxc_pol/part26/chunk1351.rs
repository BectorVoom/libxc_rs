//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1351/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1351<F: Float>(t5294: F, t6032: F, t1148: F, t19106: F, t19115: F, t19123: F, t19129: F, t19143: F, t20856: F, t20868: F, t20878: F, t20900: F, t20903: F, t20904: F, t20913: F, t20917: F, t22054: F, t22062: F, t22065: F, t22070: F, t22071: F, t22085: F, t4300: F, t4322: F, t5275: F, t5276: F, t5295: F, t6016: F, t6022: F, t6024: F, t6027: F, t6031: F, t6034: F, t63371: F, t63396: F, t63426: F, t6514: F, t6516: F, t68532: F, t73171: F, t73191: F) -> (F,) {
    let t73198 = t6032 * t5294;
    let t73206 = -t19106 * t5295 + 24.0 * t6024 * t63426 * t22054 * t1148 - t6022 * t22085 - 2.0 * t6031 * t20913 * t20903 - 2.0 * t6514 * t20917 - 6.0 * t6024 * t19123 * t22065 * t1148 + 2.0 * t73171 * t6027 + 4.0 * t19115 * t22062 - 12.0 * t6024 * t19123 * t6516 * t4322 - 2.0 * t20868 * t20900 - 6.0 * t6024 * t19123 * t6016 * t5275 - 2.0 * t63371 * t22071 - 2.0 * t19143 * t63396 * t22070 - t6031 * t73191 * t6034 + 2.0 * t19106 * t5276 + 4.0 * t68532 * t20878 + 2.0 * t19129 * t73198 * t6034 + 4.0 * t20856 * t4300 - 2.0 * t20868 * t20904;
    (t73206,)
}
