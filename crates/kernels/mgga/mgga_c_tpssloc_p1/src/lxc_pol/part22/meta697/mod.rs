//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta697 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2279;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta697<F: Float>(t18392: F, t3490: F, t1227: F, t18241: F, t248: F, t3521: F, t19040: F, t15734: F, t5024: F, t11818: F, t3515: F, t6230: F, t15578: F, t4889: F, t11789: F, t5979: F, t19051: F, t3523: F, t19080: F, t3572: F, t11709: F, t18356: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t65613, t65617, t65619, t65628, t65632) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2279::<F>(t18392, t3490, t1227, t18241, t248, t3521, t19040, t15734, t5024, t11818, t3515, t6230);
        let (t65637, t65647, t65649, t65651, t65660) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280::<F>(t15578, t4889, t11789, t1227, t248, t5979, t19051, t3523, t19080, t3572, t11709, t18356);
    (t65613, t65617, t65619, t65628, t65632, t65637, t65647, t65649, t65651, t65660)
}
