//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1457/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457<F: Float>(t44275: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78107: F, t78109: F) -> F {
    let t78853 = -F::cast_from(0.13892666666666666667e0_f64) * t78084 - F::new(0.125034e1) * t78087 + F::new(0.83356e0) * t78090 + F::new(0.375102e1) * t78093 + F::new(0.3529725e1) * t78095 + t44275 + F::new(0.94674375e0) * t78097 + F::cast_from(0.27785333333333333334e0_f64) * t78100 + F::cast_from(0.27545333333333333333e1_f64) * t63361 + F::new(0.1262325e1) * t78103 - F::new(0.705945e1) * t78105 + F::cast_from(0.158837625e2_f64) * t78107 - F::new(0.94674375e0) * t78109 - F::new(0.123954e2) * t78057;
    t78853
}
