//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2253;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2254;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2255;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2256;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2257;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2258;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta715<F: Float>(t23097: F, t232: F, t67793: F, t815: F, t2628: F, t5585: F, t776: F, t13228: F, t4233: F, t6605: F, t25119: F, t58557: F, t22690: F, t5527: F, t81792: F, t841: F, t16805: F, t1898: F, t249: F, t236: F, t5584: F, t23109: F, t2632: F, t81914: F, t23110: F, t5611: F, t81877: F, t81883: F, t87308: F, t87329: F, t98744: F, t98746: F, t98750: F, t98752: F, t98754: F, t5587: F, t81886: F, t23041: F, t5619: F, t16753: F, t16928: F, t25084: F, t16851: F, t221: F, t87420: F, t16944: F, t25154: F, t87407: F, t81903: F, t87331: F, t87333: F, t87336: F, t87339: F, t87342: F, t87348: F, t87364: F, t87387: F, t87402: F, t92652: F, t23127: F, t5628: F, t16985: F, t6621: F, t1516: F, t87321: F, t25068: F, t4261: F, t5624: F, t23133: F, t87340: F, t16673: F, t6620: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98758, t98762, t98766, t98770) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2253::<F>(t23097, t232, t67793, t815, t2628, t5585, t776, t13228, t4233, t6605, t25119, t58557);
        let (t98774, t98777, t98779, t98782) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2254::<F>(t22690, t5527, t81792, t841, t16805, t1898, t249, t236, t5584, t23109, t2632, t81914);
        let t98795 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2255::<F>(t23109, t23110, t232, t236, t5611, t98779, t81877, t81883, t87308, t87329, t98744, t98746, t98750, t98752, t98754, t98758, t98762, t98766, t98770, t98774, t98777, t98782);
        let (t98796, t98798, t98801, t98803, t98808) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2256::<F>(t5587, t81886, t23041, t5619, t16753, t6605, t815, t16928, t25084, t16851, t221, t87420);
        let t98816 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2257::<F>(t16944, t221, t25154, t16851, t841, t87407, t81903, t87331, t87333, t87336, t87339, t87342, t87348, t87364, t87387, t87402, t92652, t98796, t98798, t98801, t98803, t98808);
        let (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2258::<F>(t23127, t5628, t16985, t6621, t1516, t87321, t25068, t4261, t5624, t23133, t87340, t16673, t6620);
    (t98795, t98816, t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832)
}
