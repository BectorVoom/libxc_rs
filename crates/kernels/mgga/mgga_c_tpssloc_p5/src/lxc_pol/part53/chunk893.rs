//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 893/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk893<F: Float>(t31043: F, t8808: F, t649: F, t8717: F, t113: F, t1266: F, t1983: F, t2096: F, t31055: F, t31304: F, t32108: F, t32111: F, t32187: F, t32189: F, t32194: F, t32197: F, t32200: F, t510: F, t650: F, t652: F, t6876: F, t7057: F, t7171: F, t7218: F, t8329: F, t8607: F, t8718: F, t8774: F, t8805: F, t8809: F, t9003: F) -> (F, F, F) {
    let t32203 = t8808 * t31043;
    let t32206 = t649 * t8717;
    let t32211 = -t113 * t32108 - F::new(2.0) * t1266 * t8718 + F::new(3.0) * t1983 * t32111 + t1983 * t32187 - t1983 * t32189 - F::new(2.0) * t1983 * t32194 + F::new(2.0) * t1983 * t32203 + F::new(2.0) * t2096 * t31304 - F::new(2.0) * t32197 * t652 - F::new(4.0) * t32200 * t652 - F::new(2.0) * t32206 * t510 - t650 * t8774 + t6876 * t8805 - t6876 * t8809 - F::new(4.0) * t7057 * t9003 + F::new(6.0) * t7171 * t8607 + F::new(2.0) * t7218 * t8607 - t31055 - t8329;
    (t32203, t32206, t32211)
}
