//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta709 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2214;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2215;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2216;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2217;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2218;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta709<F: Float>(t1888: F, t25045: F, t86873: F, t214: F, t5631: F, t1880: F, t6572: F, t22986: F, t23270: F, t5657: F, t776: F, t857: F, t258: F, t5527: F, t87642: F, t6552: F, t7479: F, t87782: F, t13053: F, t13065: F, t13463: F, t17049: F, t1911: F, t25348: F, t2597: F, t2718: F, t28307: F, t28317: F, t4273: F, t7517: F, t7538: F, t855: F, t86844: F, t86869: F, t86887: F, t86896: F, t92383: F, t98117: F, t98122: F, t2717: F, t5636: F, t225: F, t28437: F, t5544: F, t25038: F, t865: F, t23237: F, t28294: F, t28267: F, t82159: F, t25054: F, t6555: F, t25216: F, t25224: F, t25040: F, t17052: F, t6663: F, t82070: F, t82082: F, t86929: F, t92406: F, t81651: F, t82074: F, t25044: F, t4300: F, t23035: F, t28298: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98125, t98133, t98135, t98148) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2214::<F>(t1888, t25045, t86873, t214, t5631, t1880, t6572, t22986, t23270, t5657, t776, t857);
        let t98160 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2215::<F>(t23270, t258, t5527, t776, t87642, t6552, t7479, t87782, t13053, t13065, t13463, t17049, t1911, t25348, t2597, t2718, t28307, t28317, t4273, t7517, t7538, t855, t86844, t86869, t86887, t86896, t92383, t98117, t98122, t98125, t98135, t98148);
        let (t98164, t98166, t98172, t98181) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2216::<F>(t2717, t5636, t22986, t23270, t776, t225, t28437, t258, t5544, t25038, t1888, t5657, t865);
        let (t98189, t98192, t98196, t98199, t98202) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2217::<F>(t1880, t23237, t28294, t22986, t28267, t82159, t25054, t86873, t6552, t6555, t98133, t25216, t25224);
        let t98208 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2218::<F>(t25038, t25040, t86873, t17052, t6663, t82070, t82082, t86929, t92406, t98189, t98192, t98196, t98199, t98202);
        let (t98213, t98222, t98227, t98234) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2219::<F>(t28267, t81651, t82074, t1888, t23270, t25044, t4300, t5527, t857, t25038, t865, t23035, t23237, t28298);
    (t98160, t98164, t98166, t98172, t98181, t98208, t98213, t98222, t98227, t98234)
}
