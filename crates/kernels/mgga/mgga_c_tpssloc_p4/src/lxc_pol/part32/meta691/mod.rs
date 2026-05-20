//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta691 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2137;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2138;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2139;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2140;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta691<F: Float>(t26179: F, t7468: F, t26003: F, t7458: F, t26142: F, t4028: F, t22674: F, t28191: F, t80681: F, t1985: F, t22666: F, t28232: F, t26331: F, t26333: F, t90566: F, t28205: F, t7700: F, t90739: F, t28206: F, t6883: F, t1385: F, t1992: F, t22635: F, t3886: F, t6460: F, t6897: F, t12021: F, t1375: F, t16460: F, t20026: F, t26477: F, t5354: F, t6439: F, t6958: F, t6992: F, t7729: F, t80663: F, t80671: F, t90460: F, t90469: F, t90471: F, t90473: F, t90498: F, t90501: F, t22892: F, t28209: F, t22685: F, t6888: F, t19631: F, t6889: F, t6890: F, t12020: F, t225: F, t28051: F, t1386: F, t20044: F, t2016: F, t28187: F, t3758: F, t56640: F, t6993: F, t90525: F, t90534: F, t90542: F, t90547: F, t90550: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t96839, t96842, t96844, t96846, t96848, t96851) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2137::<F>(t26179, t7468, t26003, t7458, t26142, t4028, t22674, t28191, t80681, t1985, t22666, t28232);
        let (t96854, t96857, t96866, t96868, t96873) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2138::<F>(t26331, t26333, t90566, t1985, t22666, t28205, t7700, t90739, t28206, t6883, t1385, t1992, t22635, t3886, t6460);
        let t96885 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2139::<F>(t22674, t28205, t6897, t12021, t1375, t16460, t20026, t26477, t5354, t6439, t6958, t6992, t7729, t80663, t80671, t90460, t90469, t90471, t90473, t90498, t90501, t96848, t96851, t96854, t96857, t96866, t96868, t96873);
        let (t96893, t96896, t96900, t96905, t96910) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2140::<F>(t22674, t22892, t28209, t22666, t22685, t28191, t6888, t19631, t6889, t6890, t12020, t1385, t1992, t22635, t6439);
        let t96917 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2141::<F>(t225, t28051, t1386, t20044, t2016, t28187, t3758, t56640, t6993, t90525, t90534, t90542, t90547, t90550, t96905, t96910);
    (t96839, t96842, t96844, t96846, t96885, t96893, t96896, t96900, t96917)
}
