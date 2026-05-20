//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1659/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1659<F: Float>(t12050: F, t12091: F, t12044: F, t12048: F, t12057: F, t12059: F, t12087: F, t12094: F, t15898: F, t15911: F, t15916: F, t15917: F, t15923: F, t19599: F, t9780: F, t9789: F) -> (F, F, F) {
    let t19677 = F::new(12.0) * t12050;
    let t19678 = F::cast_from(0.17315859105681463759e2_f64) * t12091;
    let t19679 = -t15898 + t9780 + t19599 + t12044 + t15911 - t12048 + t19677 - t15916 - t15917 - t12057 - t12059 + t15923 - t9789 + t12087 - t19678 - t12094;
    (t19677, t19678, t19679)
}
