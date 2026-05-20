//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta456 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1658;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1659;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1660;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta456<F: Float>(t225: F, t7085: F, t10110: F, t2053: F, t2719: F, t23251: F, t23261: F, t7106: F, t865: F, t2718: F, t2742: F, t10049: F, t2054: F, t23243: F, t23249: F, t23254: F, t23259: F, t23266: F, t23274: F, t2597: F, t2713: F, t2743: F, t7087: F, t7092: F, t7107: F, t855: F, t866: F, t9590: F, t9593: F, t24300: F, t870: F, t2752: F, t7109: F, t10143: F, t2056: F, t1877: F, t2057: F, t2249: F, t22951: F, t22961: F, t22964: F, t22968: F, t23296: F, t23299: F, t23302: F, t24191: F, t25: F, t2522: F, t4314: F, t606: F, t6542: F, t6671: F, t7110: F, t7114: F, t13487: F, t193: F, t202: F, t2379: F, t2553: F, t2745: F, t2749: F, t776: F, t868: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24305, t24314, t24318, t24321, t24325, t24330, t24333) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1656::<F>(t225, t7085, t10110, t2053, t2719, t23251, t23261, t7106, t865, t2718, t2742, t10049, t2054, t23243, t23249, t23254, t23259, t23266, t23274, t2597, t2713, t2743, t7087, t7092, t7107, t855, t866, t9590, t9593);
        let (t24334, t24335) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1657::<F>(t24300, t24333, t870);
        let t24339 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1658::<F>(t2752, t7109);
        let t24344 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1659::<F>(t10143, t2056);
        let t24355 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1660::<F>(t1877, t2057, t2249, t22951, t22961, t22964, t22968, t23296, t23299, t23302, t24191, t24335, t24339, t24344, t25, t2522, t4314, t606, t6542, t6671, t7110, t7114);
        let t24379 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1661::<F>(t13487, t1877, t193, t202, t2057, t2379, t24334, t24339, t24344, t2522, t2553, t2745, t2749, t4314, t7110, t7114, t776, t868, t870);
    (t24305, t24314, t24318, t24321, t24325, t24330, t24334, t24335, t24339, t24344, t24355, t24379)
}
