//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2669/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2669<F: Float>(t39512: F, t39514: F, t39522: F, t39532: F, t56374: F, t39540: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t39563: F) -> (F, F, F, F, F, F, F) {
    let t74474 = F::cast_from(0.48159733137676571078e0_f64) * t39512;
    let t74475 = F::cast_from(0.21687162600603479684e-1_f64) * t39514;
    let t74476 = F::cast_from(0.32530743900905219526e-1_f64) * t39522;
    let t74477 = F::cast_from(0.35089341735807877242e1_f64) * t39532;
    let t74478 = F::cast_from(0.54934341918019635162e-3_f64) * t56374;
    let t74479 = F::cast_from(0.5848223622634646207e0_f64) * t39540;
    let t74480 = t39499 + t39502 - t39505 - t39508 + t74474 - t74475 + t39518 - t39521 - t74476 - t39529 - t74477 + t39539 - t74478 - t74479 + t39549 + t39563;
    (t74474, t74475, t74476, t74477, t74478, t74479, t74480)
}
