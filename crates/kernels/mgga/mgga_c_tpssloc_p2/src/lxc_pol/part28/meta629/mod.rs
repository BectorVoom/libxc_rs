//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta629<F: Float>(t1877: F, t2057: F, t584: F, t9212: F, t2219: F, t7110: F, t26756: F, t86732: F, t86843: F, t86868: F, t86870: F, t225: F, t26722: F, t86886: F, t86895: F, t2053: F, t40889: F, t10049: F, t13049: F, t25168: F, t26713: F, t2743: F, t7842: F, t866: F, t86847: F, t86852: F, t86857: F, t86862: F, t86866: F, t86875: F, t86881: F, t86884: F, t86891: F, t86901: F, t86903: F, t86911: F, t86916: F, t86928: F, t86940: F, t86942: F, t13029: F, t2047: F, t259: F, t26700: F, t4142: F, t7084: F, t82079: F, t82082: F, t82087: F, t86933: F, t9590: F) -> (F, F, F, F, F, F, F, F) {
        let (t92356, t92359, t92362, t92364, t92375, t92382, t92383, t92386) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1970::<F>(t1877, t2057, t584, t9212, t2219, t7110, t26756, t86732, t86843, t86868, t86870, t225, t26722);
        let t92400 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1971::<F>(t86886, t86895, t2053, t40889, t10049, t13049, t25168, t26713, t2743, t7842, t866, t86847, t86852, t86857, t86862, t86866, t86875, t86881, t86884, t86891, t86901, t86903, t92375, t92382, t92383, t92386);
        let (t92402, t92406, t92428) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1972::<F>(t86911, t86916, t86928, t86940, t86942, t13029, t2047, t259, t26700, t2743, t4142, t7084, t7842, t82079, t82082, t82087, t86933, t9590);
    (t92356, t92359, t92362, t92364, t92400, t92402, t92406, t92428)
}
