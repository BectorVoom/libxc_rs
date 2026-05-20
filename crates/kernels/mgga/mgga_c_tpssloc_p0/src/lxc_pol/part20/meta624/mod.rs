//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta624<F: Float>(t10109: F, t1527: F, t13036: F, t225: F, t2678: F, t829: F, t828: F, t9632: F, t1519: F, t9971: F, t13336: F, t68: F, t1496: F, t41083: F, t4257: F, t9601: F, t13193: F, t2697: F, t13204: F, t2563: F, t2379: F, t40959: F, t40962: F, t40966: F, t40982: F, t40984: F, t40988: F, t40990: F, t40998: F, t4119: F, t820: F, t843: F, t9607: F, t842: F, t4261: F, t1516: F, t40965: F, t13347: F, t119: F, t13248: F, t13254: F, t13350: F, t13365: F, t210: F, t2623: F, t2643: F, t2647: F, t2703: F, t40992: F, t41009: F, t41012: F, t4172: F, t46426: F, t787: F, t849: F, t9609: F, t9990: F) -> (F, F, F, F, F, F, F, F) {
        let (t46488, t46508, t46511, t46519, t46524, t46528) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246::<F>(t10109, t1527, t13036, t225, t2678, t829, t828, t9632, t1519, t9971, t13336, t68);
        let t46560 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2247::<F>(t1496, t41083, t4257, t9601, t13193, t2697, t13204, t2563, t2379, t40959, t40962, t40966, t40982, t40984, t40988, t40990, t40998, t4119, t820, t843, t9607);
        let t46593 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2248::<F>(t4119, t828, t46528, t842, t4261, t9601, t1516, t40965, t13347, t2697, t119, t13248, t13254, t13350, t13365, t210, t2623, t2643, t2647, t2703, t40992, t41009, t41012, t4172, t46426, t787, t849, t9609, t9990);
    (t46488, t46508, t46511, t46519, t46524, t46528, t46560, t46593)
}
