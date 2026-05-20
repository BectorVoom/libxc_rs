//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2336/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2336<F: Float>(t20969: F, t2639: F, t16752: F, t2632: F, t120: F, t13222: F, t13228: F, t13251: F, t13262: F, t13350: F, t13351: F, t1512: F, t16836: F, t16839: F, t16918: F, t16932: F, t16937: F, t17017: F, t20756: F, t20986: F, t2643: F, t2645: F, t41453: F, t41467: F, t4178: F, t4180: F, t4181: F, t4255: F, t46574: F, t5612: F, t58557: F, t58765: F, t67578: F, t67607: F, t829: F) -> (F, F) {
    let t67735 = t2639 * t20969;
    let t67739 = t2632 * t16752;
    let t67777 = F::new(7.0) / F::new(4608.0) * t67735 - t58765 * t1512 / F::new(1024.0) + t4178 * t4180 * t4181 * t67739 / F::new(512.0) + t13251 * t16918 / F::new(256.0) - t13251 * t17017 / F::new(1024.0) - t16836 * t16932 / F::new(128.0) + t16836 * t16937 / F::new(256.0) + t13262 * t2645 * t67607 * t41453 / F::new(128.0) - F::new(3.0) / F::new(512.0) * t13262 * t4180 * t16839 * t67578 + F::new(5.0) / F::new(128.0) * t2643 * t41467 * t120 * t20756 * t829 + F::new(5.0) / F::new(128.0) * t4178 * t13350 * t13228 * t58557 - t4178 * t13222 * t20986 * t13351 / F::new(128.0) - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t5612 * t4255 - t46574;
    (t67739, t67777)
}
