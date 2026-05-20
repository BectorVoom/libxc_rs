//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta796 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2760;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2761;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta796<F: Float>(t40817: F, t13191: F, t13487: F, t16592: F, t16606: F, t17120: F, t1877: F, t193: F, t2378: F, t2522: F, t2553: F, t2749: F, t39549: F, t39563: F, t40772: F, t4307: F, t4310: F, t4314: F, t5664: F, t58071: F, t58080: F, t58085: F, t58090: F, t40: F, t12606: F, t12652: F, t1430: F, t16558: F, t16637: F, t16642: F, t2244: F, t2250: F, t4104: F, t5433: F, t5435: F, t55677: F, t55723: F, t607: F, t75: F, t767: F, zeta_threshold: F, t52: F, t1431: F, t16649: F, t16654: F, t4111: F, t5437: F, t5439: F, t771: F, t78: F, t17083: F, t225: F, t5584: F, t852: F, t16805: F, t68: F, t10076: F, t13171: F, t13263: F, t13381: F, t13388: F, t13390: F, t13397: F, t13456: F, t16758: F, t16816: F, t16830: F, t17030: F, t17046: F, t2633: F, t4162: F, t4281: F, t4282: F, t4290: F, t4291: F, t4292: F, t4295: F, t5612: F, t812: F, t861: F, t1509: F, t4265: F, t13336: F, t13393: F, t13450: F, t13453: F, t1510: F, t1525: F, t16756: F, t16815: F, t16817: F, t16820: F, t16825: F, t17031: F, t17034: F, t2617: F, t2679: F, t2684: F, t47395: F, t47419: F, t5651: F, t829: F, t9612: F, t1519: F, t4233: F, t2631: F, t40933: F, t13433: F, t16828: F, t17023: F, t2613: F, t4234: F, t47386: F, t5655: F, t808: F, t9632: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58094, t58095) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758::<F>(t40817, t13191, t13487, t16592, t16606, t17120, t1877, t193, t2378, t2522, t2553, t2749, t39549, t39563, t40772, t4307, t4310, t4314, t5664, t58071, t58080, t58085, t58090);
        let t58116 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759::<F>(t40, t12606, t12652, t1430, t16558, t16637, t16642, t2244, t2250, t4104, t5433, t5435, t55677, t55723, t607, t75, t767, zeta_threshold);
        let t58137 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2760::<F>(t52, t12606, t12652, t1431, t16558, t16649, t16654, t2244, t2250, t4111, t5437, t5439, t55677, t55723, t607, t771, t78, zeta_threshold);
        let t58139 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2761::<F>(t58116, t58137);
        let (t58143, t58166, t58181, t58194) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762::<F>(t17083, t225, t5584, t852, t16805, t68, t10076, t13171, t13263, t13381, t13388, t13390, t13397, t13456, t16758, t16816, t16830, t17030, t17046, t2633, t4162, t4281, t4282, t4290, t4291, t4292, t4295, t5612, t812, t861);
        let (t58204, t58224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763::<F>(t1509, t4265, t13336, t13393, t13450, t13453, t1510, t1525, t16756, t16758, t16815, t16817, t16820, t16825, t16830, t17031, t17034, t2617, t2679, t2684, t4291, t47395, t47419, t5651, t812, t829, t9612);
        let (t58226, t58246, t58261) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764::<F>(t1519, t4233, t2631, t40933, t13263, t13390, t13397, t13433, t16758, t16815, t16828, t17023, t17030, t2613, t2633, t2679, t2684, t4234, t4281, t4291, t47386, t5655, t58166, t808, t812, t829, t9632);
    (t58094, t58095, t58139, t58143, t58166, t58181, t58194, t58204, t58224, t58226, t58246, t58261)
}
