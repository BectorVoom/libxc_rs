//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2343/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2343<F: Float>(t20947: F, t776: F, t13005: F, t13222: F, t13223: F, t13251: F, t13350: F, t13365: F, t16907: F, t16985: F, t20885: F, t20972: F, t221: F, t2643: F, t41096: F, t4172: F, t4191: F, t4255: F, t5617: F, t5628: F, t58642: F, t58791: F, t58797: F, t58809: F, t58845: F, t58847: F) -> (F, F) {
    let t68010 = t20947 * t776;
    let t68018 = -F::new(7.0) / F::new(192.0) * t58791 + F::new(7.0) / F::new(96.0) * t58797 - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t13223 * t20972 - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t5617 * t4255 + t41096 + F::new(119.0) / F::new(2304.0) * t58809 + t2643 * t13222 * t13223 * t20885 / F::new(256.0) + F::new(7.0) / F::new(384.0) * t58845 + F::new(7.0) / F::new(192.0) * t58847 + t13251 * t16907 / F::new(256.0) + t58642 * t4191 / F::new(256.0) - F::new(3.0) / F::new(4.0) * t13005 * t221 * t68010 - t13365 * t5628 / F::new(256.0) - t4172 * t16985 / F::new(256.0);
    (t68010, t68018)
}
