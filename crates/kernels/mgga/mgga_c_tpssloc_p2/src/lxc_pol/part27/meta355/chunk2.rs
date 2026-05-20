//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1471/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1471<F: Float>(t13160: F, t776: F, t2553: F, t4226: F, t12971: F, t824: F, t13141: F, t13151: F, t13157: F, t1504: F, t1506: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4219: F, t4225: F, t4227: F, t4230: F, t822: F, t825: F) -> F {
    let t13161 = t13160 * t776;
    let t13164 = t4226 * t2553;
    let t13167 = t824 * t12971;
    let t13170 = -t13141 * t230 - F::new(24.0) * t13151 * t4227 + F::new(60.0) * t13157 * t4225 - F::new(24.0) * t13161 * t4225 - F::new(12.0) * t13164 * t4225 + F::new(3.0) * t13167 * t228 - F::new(12.0) * t1504 * t2672 + F::new(3.0) * t1504 * t2675 + F::new(3.0) * t1506 * t2667 + F::new(6.0) * t4219 * t825 + F::new(6.0) * t4230 * t822;
    t13170
}
