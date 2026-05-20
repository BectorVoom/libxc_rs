//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2966/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966<F: Float>(t10408: F, t10413: F, t13977: F, t13982: F, t13987: F, t13991: F, t14099: F, t14103: F, t14508: F, t14511: F, t17673: F, t17693: F, t3041: F, t3048: F, t3070: F, t3071: F, t42432: F, t42561: F, t4347: F, t4650: F, t48548: F, t48564: F, t48567: F, t48570: F, t48574: F, t50265: F, t5677: F) -> F {
    let t61835 = t3070 * t3071 * t4650 * t4347 / F::new(1152.0) + F::new(5.0) / F::new(10368.0) * t48548 + t14508 * t13982 / F::new(768.0) + t48570 * t13987 / F::new(256.0) - t50265 * t13991 / F::new(256.0) - t14511 * t14103 / F::new(1536.0) - t42561 * t17673 / F::new(48.0) + t14508 * t13977 / F::new(384.0) - t14511 * t14099 / F::new(768.0) + t48564 / F::new(576.0) - F::new(5.0) / F::new(648.0) * t3048 * t17693 + F::new(5.0) / F::new(10368.0) * t48567 + t48574 / F::new(2304.0) - t42432 / F::new(20736.0) - F::new(5.0) / F::new(13824.0) * t10413 * t10408 * t5677 * t3041;
    t61835
}
