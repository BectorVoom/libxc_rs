//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 707/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk707<F: Float>(t1895: F, t794: F, t23103: F, t1899: F, t2693: F, t281: F, t6598: F, t22690: F, t814: F) -> (F, F, F, F, F, F) {
    let t23104 = t794 * t1895;
    let t23105 = t23103 * t23104;
    let t23106 = F::new(0.16821981705891829522e-4) * t23105;
    let t23107 = t1899 * t2693;
    let t23108 = F::new(119.0) / F::new(6912.0) * t23107;
    let t23109 = t6598 * t281;
    let t23110 = t22690 * t814;
    (t23105, t23106, t23107, t23108, t23109, t23110)
}
