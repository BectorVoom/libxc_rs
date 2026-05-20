//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta598<F: Float>(t26739: F, t2752: F, t193: F, t201: F, t7844: F, t86843: F, t86868: F, t225: F, t26722: F, t86886: F, t86895: F, t2053: F, t40889: F) -> (F, F, F, F, F, F, F, F) {
        let (t92276, t92319, t92375, t92382, t92386, t92390, t92393, t92394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1843::<F>(t26739, t2752, t193, t201, t7844, t86843, t86868, t225, t26722, t86886, t86895, t2053, t40889);
    (t92276, t92319, t92375, t92382, t92386, t92390, t92393, t92394)
}
