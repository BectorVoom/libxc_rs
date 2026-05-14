//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1331/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1331<F: Float>(t1107: F, t1883: F, t2710: F, t68222: F, t20853: F, t1148: F, t1149: F, t12636: F, t1880: F, t19106: F, t19113: F, t19115: F, t19118: F, t19123: F, t19125: F, t19151: F, t20856: F, t20865: F, t20868: F, t20873: F, t20882: F, t20887: F, t20900: F, t20904: F, t20914: F, t20917: F, t3119: F, t3144: F, t3145: F, t4300: F, t6022: F, t6024: F, t6025: F, t6031: F, t6034: F, t63419: F, t63426: F, t6509: F, t6516: F, t6522: F, t6525: F, t68192: F) -> (F,) {
    let t68224 = t1883 * t2710 * t68222 * t1107;
    let t68235 = t1107 * t20853;
    let t68243 = -6.0 * t20865 * t19125 - 2.0 * t68192 * t1149 - 12.0 * t6024 * t19123 * t20882 * t1148 - t63419 * t6522 + 2.0 * t6024 * t6025 * t6509 * t3144 + 4.0 * t19115 * t20887 - t20856 * t3145 - 2.0 * t19118 * t20900 - 2.0 * t19118 * t20904 - 6.0 * t6024 * t19123 * t6516 * t3144 + 4.0 * t19106 * t4300 + 2.0 * t6024 * t6025 * t1880 * t12636 - 12.0 * t68224 * t20873 * t6034 - t19113 * t6525 - 2.0 * t6022 * t20917 + 24.0 * t6024 * t63426 * t6516 * t3119 - 2.0 * t6031 * t68235 * t6034 - 2.0 * t19118 * t20914 - 2.0 * t20868 * t19151;
    (t68243,)
}
