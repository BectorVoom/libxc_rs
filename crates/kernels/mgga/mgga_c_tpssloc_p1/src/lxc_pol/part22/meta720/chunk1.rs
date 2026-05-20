//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2335/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2335<F: Float>(t20887: F, t9638: F, t13242: F, t13251: F, t13254: F, t16839: F, t16903: F, t16935: F, t20972: F, t20974: F, t20983: F, t20986: F, t20988: F, t2632: F, t2643: F, t2645: F, t4119: F, t4178: F, t4180: F, t58480: F, t58482: F, t58504: F, t58528: F, t67607: F, t9627: F, t9642: F, t9646: F) -> F {
    let t67729 = t9638 * t20887;
    let t67732 = -F::new(7.0) / F::new(192.0) * t58480 + F::new(7.0) / F::new(768.0) * t58482 + t13251 * t16903 / F::new(256.0) - t4178 * t2645 * t67607 * t9627 / F::new(128.0) + F::new(3.0) / F::new(512.0) * t4178 * t4180 * t16839 * t16935 - F::new(5.0) / F::new(256.0) * t9642 * t20974 - F::new(5.0) / F::new(256.0) * t2643 * t9646 * t13242 * t20972 - t13254 * t20983 / F::new(128.0) - t4178 * t2645 * t16839 * t2632 * t4119 / F::new(128.0) + t13254 * t20988 / F::new(512.0) + t4178 * t4180 * t13242 * t20986 / F::new(512.0) - F::new(7.0) / F::new(192.0) * t58504 - F::new(7.0) / F::new(384.0) * t67729 + F::new(7.0) / F::new(48.0) * t58528;
    t67732
}
