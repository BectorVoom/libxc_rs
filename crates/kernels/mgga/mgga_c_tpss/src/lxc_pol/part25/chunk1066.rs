//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1066/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1066<F: Float>(t14632: F, t895: F, t904: F, t912: F, t2629: F, t4961: F, t11399: F, t3907: F, t10980: F, t11002: F, t11134: F, t11135: F, t14459: F, t14492: F, t14495: F, t14505: F, t14507: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t8616: F, t8723: F) -> (F, F, F, F) {
    let t14634 = t895 * t14632 * t904;
    let t14636 = F::new(0.5848223622634646207e0) * t912 * t14634;
    let t14638 = F::new(0.17315859105681463759e2) * t2629 * t4961;
    let t14639 = t3907 * t11399;
    let t14641 = F::new(0.34631718211362927518e2) * t912 * t14639;
    let t14656 = -t8723 - F::new(0.79148148148148148147e-2) * t8616 - F::new(0.15829629629629629629e-1) * t10980 + F::new(0.79148148148148148147e-2) * t11002 - t11134 + t11135 + F::new(0.39574074074074074073e-2) * t14495 - F::new(0.19787037037037037037e-1) * t14517 + F::new(0.71233333333333333332e-1) * t14459 - F::new(0.23744444444444444444e-1) * t14521 - F::new(0.11872222222222222222e-1) * t14505 - F::new(0.10685e0) * t14525 + F::new(0.71233333333333333332e-1) * t14528 + F::new(0.5936111111111111111e-2) * t14507 - F::new(0.11872222222222222222e-1) * t14532 + F::new(0.35616666666666666666e-1) * t14535 - F::new(0.17808333333333333333e-1) * t14492;
    (t14636, t14638, t14641, t14656)
}
