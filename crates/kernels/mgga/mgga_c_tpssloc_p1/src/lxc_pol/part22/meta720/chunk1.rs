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
    let t67732 = -F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t58480 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t58482 + t13251 * t16903 / F::cast_from(256.0_f64) - t4178 * t2645 * t67607 * t9627 / F::cast_from(128.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t4178 * t4180 * t16839 * t16935 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t9642 * t20974 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t9646 * t13242 * t20972 - t13254 * t20983 / F::cast_from(128.0_f64) - t4178 * t2645 * t16839 * t2632 * t4119 / F::cast_from(128.0_f64) + t13254 * t20988 / F::cast_from(512.0_f64) + t4178 * t4180 * t13242 * t20986 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t58504 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t67729 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t58528;
    t67732
}
