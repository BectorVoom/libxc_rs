//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1957/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957<F: Float>(t109: F, t84036: F, t86583: F, t86586: F, t92122: F, t92123: F, t96713: F, t96716: F, t96719: F, t96721: F, t96724: F, t96726: F, t2098: F, t671: F) -> (F, F) {
    let t110 = F::new(1.0) < t109;
    let t100989 = -t84036 - t86583 - F::new(44.0) / F::new(9.0) * t86586 - t92122 + t92123 - F::new(4.0) / F::new(3.0) * t96713 - F::new(3.0) / F::new(2.0) * t96716 + t96719 + F::new(2.0) / F::new(3.0) * t96721 + t96724 / F::new(2.0) - t96726 / F::new(4.0);
    let t100990 = piecewise3::<F>(t110, F::new(0.0), t100989);
    let t100993 = t2098 * t671;
    (t100990, t100993)
}
