//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1283/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1283<F: Float>(t103581: F, t103687: F, t103710: F, t103723: F, t103767: F, t103774: F, t1244: F, t1246: F, t1653: F, t1734: F, t22368: F, t24812: F, t24815: F, t24849: F, t27507: F, t29664: F, t29716: F, t29727: F, t29749: F, t29753: F, t29754: F, t3610: F, t7373: F, t7376: F, t7999: F, t8073: F, t8074: F, t8082: F, t86037: F, t94858: F, t94932: F, t94936: F, t94966: F) -> (F,) {
    let t109244 = 0.65797362673929057459e-1 * t94858 * t29754 + 0.24125699647107321069e0 * t103581 * t8074 + 3.0 * t1244 * t29664 * t1734 * t1246 - 0.49348022005446793095e-1 * t24812 * t94932 * t29749 - 0.24674011002723396548e-1 * t7373 * t103687 * t8073 - 0.24674011002723396548e-1 * t7373 * t103723 * t8073 + 0.24674011002723396548e-1 * t24812 * t94936 * t29753 - 0.16449340668482264365e-1 * t86037 * t103774 * t24815 * t1653 - 0.82246703342411321826e-2 * t24849 * t103767 * t7376 * t1653 + 0.18277045187202515961e-2 * t94966 - 0.54831135561607547883e-2 * t103710 + 0.13159472534785811492e0 * t27507 * t29716 - 0.65797362673929057459e-1 * t7999 * t29727 + 6.0 * t3610 * t8082 * t22368;
    (t109244,)
}
