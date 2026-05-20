//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta423 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1738;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1739;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1740;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1741;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta423<F: Float>(t11665: F, t11678: F, t1174: F, t11834: F, t1218: F, t15569: F, t15717: F, t15719: F, t15722: F, t15740: F, t18997: F, t19002: F, t19005: F, t19010: F, t19016: F, t19019: F, t19026: F, t3577: F, t4889: F, t4950: F, t4954: F, t4969: F, t5046: F, t6192: F, t372: F, t6163: F, t479: F, t471: F, t248: F, t3521: F, t5979: F, t1227: F, t1009: F, t6150: F, t1011: F, t1212: F, t1226: F, t6169: F, t486: F, t6218: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t19029 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1738::<F>(t11665, t11678, t1174, t11834, t1218, t15569, t15717, t15719, t15722, t15740, t18997, t19002, t19005, t19010, t19016, t19019, t19026, t3577, t4889, t4950, t4954, t4969, t5046, t6192);
        let (t19032, t19033) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1739::<F>(t372, t6163, t479, t471);
        let (t19040, t19041, t19045, t19046, t19047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1740::<F>(t248, t3521, t5979, t1227, t1009, t6150, t1011, t1212);
        let t19051 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1741::<F>(t1226, t6169);
        let t19056 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1742::<F>(t486, t6218);
    (t19029, t19032, t19033, t19040, t19041, t19045, t19046, t19047, t19051, t19056)
}
