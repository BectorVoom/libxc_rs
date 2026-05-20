//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2249;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2251;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2253;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta625<F: Float>(t13210: F, t9638: F, t120: F, t13170: F, t2553: F, t828: F, t13231: F, t13258: F, t41107: F, t4250: F, t13244: F, t242: F, t812: F, t841: F, t10003: F, t13222: F, t13228: F, t13229: F, t13251: F, t13300: F, t13353: F, t16935: F, t2633: F, t2643: F, t2645: F, t41025: F, t41031: F, t41467: F, t4178: F, t4180: F, t4182: F, t4248: F, t829: F, t9616: F, t9642: F, t1484: F, t2678: F, t41115: F, t4166: F, t9637: F, t2649: F, t13257: F, t2617: F, t4184: F, t4257: F, t9993: F, t13176: F, t2638: F, t831: F, t13350: F, t2647: F, t41048: F, t41050: F, t41053: F, t41055: F, t41063: F, t4191: F, t9623: F, t9661: F, t9990: F, t13278: F, t2681: F, t4236: F, t9674: F, t13186: F, t2697: F, t13289: F, t41011: F, t4179: F, t820: F, t1509: F, t13225: F, t13177: F, t13242: F, t13254: F, t13262: F, t13263: F, t1495: F, t210: F, t2686: F, t40971: F, t41161: F, t4181: F, t843: F, t9458: F, t13312: F, t4240: F, t13261: F, t836: F, t9972: F, t13265: F, t13333: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46595, t46597, t46606, t46611, t46616, t46618, t46628) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2249::<F>(t13210, t9638, t120, t13170, t2553, t828, t13231, t13258, t41107, t4250, t13244, t242, t812, t841);
        let t46637 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250::<F>(t10003, t13222, t13228, t13229, t13251, t13300, t13353, t16935, t2633, t2643, t2645, t41025, t41031, t41467, t4178, t4180, t4182, t4248, t46595, t46597, t46606, t46611, t46616, t46618, t46628, t829, t9616, t9642);
        let (t46644, t46650, t46658, t46661, t46663) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2251::<F>(t1484, t2678, t41115, t4250, t4166, t9637, t2649, t13257, t2617, t4184, t4257, t9993);
        let t46670 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252::<F>(t13176, t2638, t831, t13251, t13350, t2643, t2645, t2647, t41048, t41050, t41053, t41055, t41063, t4191, t4248, t4257, t46644, t46650, t46658, t46661, t46663, t9623, t9661, t9990);
        let (t46675, t46677, t46679, t46686, t46692, t46693) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2253::<F>(t13278, t2681, t4236, t9674, t13186, t2697, t13289, t41011, t4179, t820, t1509, t2678);
        let t46716 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254::<F>(t13225, t9638, t13177, t13222, t13231, t13242, t13254, t13262, t13263, t1484, t1495, t210, t2643, t2686, t40971, t41161, t4180, t4181, t46644, t46675, t46677, t46679, t46686, t46692, t46693, t820, t829, t843, t9458, t9642, t9661);
        let (t46717, t46733, t46737, t46742, t46748) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2255::<F>(t13312, t9638, t41107, t4240, t13261, t2617, t812, t836, t9972, t13265, t13258, t13333);
    (t46597, t46637, t46644, t46670, t46692, t46693, t46716, t46717, t46733, t46737, t46742, t46748)
}
