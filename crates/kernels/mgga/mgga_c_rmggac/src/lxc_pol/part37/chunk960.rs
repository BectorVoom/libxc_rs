//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 960/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk960<F: Float>(t77366: F, t2329: F, t71400: F, t14585: F, t8562: F, t2333: F, t71608: F, t2344: F, t71198: F, t14580: F, t1679: F, t2136: F) -> (F, F, F, F, F, F) {
    let t77367 = F::new(0.68186654135613354322e-2) * t77366;
    let t77369 = t71400 * t2329;
    let t77370 = F::new(0.13637330827122670864e-1) * t77369;
    let t77371 = t14585 * t8562;
    let t77372 = F::new(0.13637330827122670864e-1) * t77371;
    let t77373 = t71608 * t2333;
    let t77374 = F::new(0.68186654135613354322e-2) * t77373;
    let t77375 = t71198 * t2344;
    let t77376 = F::new(0.10227998120342003148e-1) * t77375;
    let t77377 = t1679 * t14580;
    let t77378 = t77377 * t2136;
    (t77367, t77370, t77372, t77374, t77376, t77378)
}
