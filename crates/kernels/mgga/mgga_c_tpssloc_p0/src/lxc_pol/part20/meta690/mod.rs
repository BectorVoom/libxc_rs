//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2620;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta690<F: Float>(t11718: F, t52835: F, t11797: F, t5024: F, t11147: F, t15394: F, t11665: F, t11724: F, t11774: F, t15455: F, t15459: F, t15463: F, t3447: F, t3490: F, t45108: F, t45112: F, t45126: F, t45148: F, t45971: F, t5005: F, t1174: F, t5045: F, t698: F, t3540: F, t4966: F, t11647: F, t1744: F, t11825: F, t45167: F, t45169: F, t45171: F, t45178: F, t45181: F, t45184: F, t4974: F, t11697: F, t15469: F, t3577: F, t11801: F, t3247: F, t475: F, t15032: F, t3576: F, t11713: F, t11716: F, t53081: F, t11148: F, t11674: F, t11678: F, t11766: F, t11855: F, t1216: F, t14706: F, t15470: F, t15661: F, t15663: F, t15740: F, t1735: F, t18946: F, t3578: F, t3580: F, t45128: F, t45162: F, t45211: F, t4889: F, t5019: F, t11786: F, t3509: F, t607: F, t3032: F, t52434: F, t3505: F, t1090: F, t1196: F, t15525: F, t15591: F, t3252: F, t3496: F, t3511: F, t45222: F, t45224: F, t45227: F, t45872: F, t4728: F, t5002: F, t5012: F, t974: F) -> (F, F, F, F, F, F, F, F, F) {
        let t53258 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618::<F>(t11718, t52835, t11797, t5024, t11147, t15394, t11665, t11724, t11774, t15455, t15459, t15463, t3447, t3490, t45108, t45112, t45126, t45148, t45971, t5005);
        let t53276 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2619::<F>(t11797, t5005, t1174, t5045, t698, t3540, t4966, t11647, t1744, t11825, t45167, t45169, t45171, t45178, t45181, t45184, t4974);
        let (t53287, t53291, t53298, t53322, t53336) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2620::<F>(t11697, t15469, t3577, t11801, t5005, t3247, t475, t15032, t3576, t11713, t11716, t53081);
        let t53345 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2621::<F>(t11148, t11665, t11674, t11678, t11724, t11766, t11855, t1216, t14706, t15470, t15661, t15663, t15740, t1735, t18946, t3577, t3578, t3580, t45128, t45162, t45211, t4889, t5019, t53322, t53336);
        let (t53366, t53371, t53377) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2622::<F>(t11786, t5024, t3509, t607, t3032, t52434, t3505, t1090, t11678, t1174, t11855, t1196, t15525, t15591, t3252, t3496, t3511, t3577, t3578, t45222, t45224, t45227, t45872, t4728, t5002, t5012, t974);
    (t53258, t53276, t53287, t53291, t53298, t53345, t53366, t53371, t53377)
}
