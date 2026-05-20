//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1792/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1792<F: Float>(t13229: F, t232: F, t815: F, t23097: F, t1891: F, t22813: F, t22816: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F) -> (F, F, F, F, F, F, F, F) {
    let t23098 = t13229 * t232;
    let t23099 = t815 * t23098;
    let t23100 = t23097 * t23099;
    let t23102 = t22813 * t1891;
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23106 = F::cast_from(0.16821981705891829522e-4_f64) * t23105;
    let t23107 = t1899 * t2693;
    let t23108 = F::new(119.0) / F::new(6912.0) * t23107;
    let t23109 = t6598 * t281;
    (t23098, t23099, t23100, t23102, t23104, t23106, t23108, t23109)
}
