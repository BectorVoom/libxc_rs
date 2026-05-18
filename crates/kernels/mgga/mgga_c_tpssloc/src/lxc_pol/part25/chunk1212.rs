//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1212/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1212<F: Float>(t10049: F, t10103: F, t10116: F, t2053: F, t2054: F, t24297: F, t24330: F, t2597: F, t2718: F, t2743: F, t40852: F, t40870: F, t41554: F, t7087: F, t7107: F, t82099: F, t82108: F, t84949: F, t84981: F, t85007: F, t85031: F, t855: F, t858: F) -> F {
    let t85047 = -F::new(3.0) * t24297 * t2743 - F::new(3.0) * t40870 * t2054 + F::new(0.15626873635058151147e0) * t82099 + F::new(6.0) * t7087 * t10116 - t40852 * t2054 - t855 * t858 * (t84949 + t84981 + t85007 + t85031) + F::new(6.0) * t2597 * t24330 + F::new(2.0) * t855 * t2718 * t2053 * t10103 - F::new(3.0) * t10049 * t7107 - F::new(3.0) * t41554 * t2054 - F::new(0.14804406601634037928e0) * t82108;
    t85047
}
