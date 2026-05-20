//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta455 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1314;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1315;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta455<F: Float>(t59028: F, t145: F, t185: F, t75929: F, t39658: F, t41258: F, t41262: F, t76024: F, t76025: F, t76026: F, t76027: F, t76030: F, t76031: F, t76034: F, t5527: F, t5544: F, t1504: F, t1506: F, t16729: F, t16736: F, t20800: F, t20835: F, t20843: F, t20846: F, t20849: F, t225: F, t228: F, t230: F, t2671: F, t41315: F, t4225: F, t4226: F, t5601: F, t5605: F, t5608: F, t75978: F, t76006: F, t76007: F, t76009: F, t76010: F, t76013: F, t76014: F, t76021: F, t824: F, t232: F, t5584: F, t40933: F, t9975: F, t13251: F, t13262: F, t1484: F, t16839: F, t16891: F, t20885: F, t20887: F, t20972: F, t2632: F, t2643: F, t2645: F, t4178: F, t4180: F, t5591: F, t5617: F, t67607: F, t67612: F, t67625: F, t67637: F, t67639: F, t68246: F, t9646: F, t119: F, t16836: F, t20974: F, t20978: F, t20986: F, t20988: F, t210: F, t2571: F, t2701: F, t41161: F, t46546: F, t58421: F, t67620: F, t67660: F, t67675: F, t820: F, t843: F, t1516: F, t16976: F, t20896: F, t20908: F, t40971: F, t4172: F, t46577: F, t5624: F, t5628: F, t58550: F, t67690: F, t67692: F, t67729: F, t67735: F, t68203: F, t847: F, t1510: F, t20756: F, t20852: F, t20882: F, t20891: F, t20983: F, t41467: F, t4181: F, t5587: F, t5593: F, t58574: F, t58576: F, t58642: F, t58811: F, t67852: F, t67854: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t76035, t76037, t76038) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313::<F>(t59028, t145, t185, t75929, t39658, t41258, t41262, t76024, t76025, t76026, t76027, t76030, t76031, t76034);
        let (t76056, t76063, t76073) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1314::<F>(t5527, t5544, t1504, t1506, t16729, t16736, t20800, t20835, t20843, t20846, t20849, t225, t228, t230, t2671, t41315, t4225, t4226, t5601, t5605, t5608, t75978, t76006, t76007, t76009, t76010, t76013, t76014, t76021, t76038, t824);
        let (t76074, t76085, t76086, t76090, t76132) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1315::<F>(t232, t76073, t5584, t40933, t9975, t13251, t13262, t1484, t16839, t16891, t20885, t20887, t20972, t2632, t2643, t2645, t4178, t4180, t5527, t5591, t5617, t67607, t67612, t67625, t67637, t67639, t68246, t9646);
        let t76167 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1316::<F>(t119, t13251, t16836, t16839, t20885, t20974, t20978, t20986, t20988, t210, t2571, t2643, t2645, t2701, t41161, t4178, t4180, t46546, t5591, t58421, t67620, t67660, t67675, t76056, t76063, t820, t843);
        let t76193 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1317::<F>(t1516, t16976, t20896, t20908, t40971, t4172, t46577, t5624, t5628, t58550, t67690, t67692, t67729, t67735, t68203, t75978, t76056, t820, t843, t847);
        let t76227 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1318::<F>(t13251, t1510, t16836, t16839, t20756, t20852, t20882, t20891, t20983, t232, t2632, t2643, t2645, t41467, t4178, t4180, t4181, t5544, t5587, t5593, t58574, t58576, t58642, t58811, t67620, t67852, t67854);
    (t76035, t76037, t76056, t76063, t76074, t76085, t76086, t76090, t76132, t76167, t76193, t76227)
}
