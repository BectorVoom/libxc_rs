//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2601/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2601<F: Float>(t1174: F, t14753: F, t3431: F, t14744: F, t11651: F, t15438: F, t1227: F, t13969: F, t15540: F, t15530: F, t3515: F, t11638: F, t11688: F, t15740: F, t3506: F, t3508: F, t44621: F, t44886: F, t44890: F, t44894: F, t4582: F, t4977: F, t50924: F) -> F {
    let t52773 = t1174 * t3431 * t14753;
    let t52776 = t1174 * t3431 * t14744;
    let t52781 = t15438 * t11651;
    let t52792 = t1227 * t13969 * t15540;
    let t52795 = t3515 * t13969 * t15530;
    let t52797 = -t15740 * t11688 / F::cast_from(768.0_f64) - t52773 / F::cast_from(144.0_f64) - t52776 / F::cast_from(48.0_f64) + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1174 * t44621 * t50924 - t52781 / F::cast_from(1536.0_f64) + t3506 * t4582 * t4977 * t3508 * t11638 / F::cast_from(1536.0_f64) - t44886 / F::cast_from(4608.0_f64) - t44890 / F::cast_from(2304.0_f64) + t44894 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t52792 - t52795 / F::cast_from(1536.0_f64);
    t52797
}
