//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2142;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2143;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2144;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2145;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2146;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2147;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2148;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2149;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2150;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2151;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2152;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta647<F: Float>(t87618: F, t1902: F, t4233: F, t1888: F, t232: F, t47528: F, t6646: F, t13398: F, t82018: F, t13404: F, t22996: F, t7521: F, t81632: F, t23035: F, t2379: F, t25319: F, t6637: F, t1887: F, t81959: F, t25248: F, t25249: F, t4265: F, t828: F, t22690: F, t23171: F, t2613: F, t4291: F, t7535: F, t81697: F, t81704: F, t81717: F, t829: F, t87609: F, t87613: F, t87615: F, t2632: F, t87106: F, t23143: F, t7525: F, t25238: F, t6579: F, t23153: F, t4119: F, t6552: F, t12971: F, t6638: F, t22893: F, t23164: F, t25312: F, t82011: F, t47425: F, t13336: F, t1909: F, t25269: F, t2617: F, t4182: F, t4281: F, t7533: F, t81980: F, t81989: F, t82005: F, t82013: F, t82016: F, t9612: F, t25038: F, t776: F, t87130: F, t22986: F, t87111: F, t82039: F, t25273: F, t244: F, t268: F, t6559: F, t25250: F, t87202: F, t25316: F, t82038: F, t47439: F, t23110: F, t23185: F, t25272: F, t25325: F, t6547: F, t13390: F, t23016: F, t25255: F, t25262: F, t25295: F, t2679: F, t2684: F, t4162: F, t4166: F, t6660: F, t808: F, t812: F, t82028: F, t82032: F, t82047: F, t1912: F, t46452: F, t82143: F, t82145: F, t82150: F, t855: F, t858: F, t87029: F, t87033: F, t87039: F, t87042: F, t87047: F, t87050: F, t87094: F, t87146: F, t87524: F, t87562: F, t87606: F, t1880: F, t7488: F, t82124: F, t1911: F, t40889: F, t25045: F, t82074: F, t254: F, t799: F, t225: F, t25161: F, t23270: F, t25039: F, t23218: F, t25224: F, t6562: F, t6572: F, t86893: F, t23228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87619, t87620, t87627, t87630, t87633, t87635) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2142::<F>(t87618, t1902, t4233, t1888, t232, t47528, t6646, t13398, t82018, t13404, t22996, t7521, t81632);
        let (t87640, t87642, t87645, t87650) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2143::<F>(t23035, t2379, t25319, t6637, t1887, t81959, t25248, t25249, t1888, t232, t4265, t6646, t828);
        let t87656 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2144::<F>(t22690, t23171, t25319, t2613, t4291, t7535, t81697, t81704, t81717, t829, t87609, t87613, t87615, t87619, t87620, t87627, t87630, t87633, t87635, t87640, t87645, t87650);
        let (t87660, t87666, t87669, t87672) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2145::<F>(t1888, t22996, t2632, t87106, t23143, t7525, t25238, t6579, t23153, t4119, t6552, t6637);
        let (t87676, t87680, t87687, t87692) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2146::<F>(t12971, t6552, t6637, t6638, t22893, t23164, t25312, t82011, t1888, t232, t47425, t6646);
        let t87694 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2147::<F>(t13336, t1909, t25269, t2617, t4182, t4281, t7533, t81980, t81989, t82005, t82013, t82016, t87620, t87660, t87666, t87669, t87672, t87676, t87680, t87687, t87692, t9612);
        let (t87699, t87705, t87708, t87710, t87712) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2148::<F>(t25038, t25248, t776, t87130, t22986, t6646, t829, t87111, t82039, t25273, t6579, t244, t268, t6559);
        let (t87714, t87718, t87726, t87729) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2149::<F>(t25250, t87202, t87712, t25316, t82038, t1888, t232, t47439, t6646, t23110, t23185, t25272);
        let t87735 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2150::<F>(t87729, t25325, t6547, t13390, t23016, t25255, t25262, t25295, t2679, t2684, t4162, t4166, t6660, t808, t812, t82028, t82032, t82047, t87699, t87705, t87708, t87710, t87714, t87718, t87726);
        let t87741 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2151::<F>(t1912, t46452, t82143, t82145, t82150, t855, t858, t87029, t87033, t87039, t87042, t87047, t87050, t87094, t87146, t87524, t87562, t87606, t87656, t87694, t87735);
        let (t87746, t87748, t87754, t87755, t87758) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2152::<F>(t1880, t7488, t82124, t1911, t40889, t23185, t25045, t82074, t254, t799, t225, t25161);
        let (t87765, t87773, t87777, t87779) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2153::<F>(t23270, t2379, t25039, t87642, t1880, t23218, t25224, t6562, t6572, t86893, t23171, t23228, t7488);
    (t87712, t87741, t87746, t87748, t87754, t87755, t87758, t87765, t87773, t87777, t87779)
}
