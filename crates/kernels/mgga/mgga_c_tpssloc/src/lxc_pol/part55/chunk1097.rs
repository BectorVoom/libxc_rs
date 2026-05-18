//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1097/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1097<F: Float>(t10110: F, t32795: F, t1911: F, t7537: F, t2718: F, t1527: F, t8362: F, t225: F, t258: F, t7510: F, t214: F, t1880: F) -> (F, F, F, F, F, F) {
    let t32796 = t10110 * t32795;
    let t32799 = t1911 * t7537;
    let t32800 = t2718 * t32799;
    let t32803 = t8362 * t1527;
    let t32804 = t2718 * t32803;
    let t32808 = t7510 * t225 * t258;
    let t32809 = t214 * t32808;
    let t32811 = F::new(0.16449340668482264365e-1) * t1880 * t32809;
    (t32796, t32800, t32804, t32808, t32809, t32811)
}
