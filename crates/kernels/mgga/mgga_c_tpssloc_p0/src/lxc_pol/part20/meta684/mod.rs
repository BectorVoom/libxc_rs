//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2592;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta684<F: Float>(t11712: F, t11880: F, t491: F, t1734: F, t6739: F, t3609: F, t52434: F, t3507: F, t5052: F, t1215: F, t2250: F, t475: F, t2244: F, t3242: F, t1216: F, t3493: F, t1011: F, t1212: F, t52446: F, t11539: F, t1174: F, t14736: F, t1227: F, t13969: F, t15544: F, t15655: F, t15636: F, t3515: F, t44571: F, t4724: F, t11778: F, t43791: F, t11720: F, t11722: F, t11748: F, t15498: F, t3587: F, t44725: F, t44811: F, t44863: F, t45030: F, t4582: F, t48497: F, t4889: F, t4977: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t52479, t52480, t52485, t52500, t52532) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2592::<F>(t11712, t11880, t491, t1734, t6739, t3609, t52434, t3507, t5052, t1215, t2250, t475);
        let (t52538, t52548, t52554, t52568, t52575) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2593::<F>(t1215, t2244, t475, t3242, t1216, t3493, t1011, t1212, t52446, t11539, t1174, t14736);
        let t52606 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2594::<F>(t1227, t13969, t15544, t15655, t15636, t3515, t1174, t44571, t4724, t11778, t43791, t11720, t11722, t11748, t15498, t3587, t44725, t44811, t44863, t45030, t4582, t48497, t4889, t4977, t52575);
    (t52479, t52480, t52485, t52500, t52532, t52538, t52548, t52554, t52568, t52606)
}
