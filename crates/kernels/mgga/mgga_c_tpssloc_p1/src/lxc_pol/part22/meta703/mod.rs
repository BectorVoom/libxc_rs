//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta703 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta703<F: Float>(t15689: F, t4889: F, t1174: F, t135: F, t18996: F, t15743: F, t5024: F, t18363: F, t3577: F, t45124: F, t11697: F, t18359: F, t15572: F, t15740: F, t18382: F, t1215: F, t6224: F, t1227: F, t13969: F, t18954: F, t19067: F, t1222: F, t18297: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t66273, t66276, t66324, t66334, t66337) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290::<F>(t15689, t4889, t1174, t135, t18996, t15743, t5024, t18363, t3577, t45124, t11697, t18359);
        let (t66360, t66363, t66388, t66398, t66406, t66408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291::<F>(t15572, t15740, t11697, t18382, t3577, t1215, t6224, t1227, t13969, t18954, t19067, t1222, t18297);
    (t66273, t66276, t66324, t66334, t66337, t66360, t66363, t66388, t66398, t66406, t66408)
}
