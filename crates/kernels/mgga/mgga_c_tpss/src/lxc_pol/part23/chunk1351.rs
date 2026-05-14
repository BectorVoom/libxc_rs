//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1351/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1351<F: Float>(t1906: F, t4543: F, t1276: F, t1278: F, t1284: F, t13292: F, t1673: F, t1902: F, t19280: F, t20986: F, t21007: F, t3399: F, t3413: F, t4544: F, t6071: F, t63667: F, t63669: F, t6548: F, t6556: F, t67886: F, t67888: F, t68752: F, t68769: F) -> (F,) {
    let t68773 = 2.0 * t4543 * t1906;
    let t68774 = t3399 * t6556 + t19280 * t1673 + t1902 * t13292 + t67886 + t67888 + 2.0 * t1276 * t21007 + 2.0 * t63669 + 2.0 * t20986 * t1284 + t6548 * t3413 + 2.0 * t4544 * t6071 + t1278 * (t68752 + t68769) + t63667 + t68773;
    (t68774,)
}
