//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1014/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1014<F: Float>(t78367: F, t75907: F, t75910: F, t70104: F, t70106: F, t70108: F, t70110: F, t75921: F, t75936: F, t75943: F, t739: F, t78112: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t78368 = F::cast_from(0.42564599893297839398e-5_f64) * t78367;
    let t78371 = F::cast_from(0.1276937996798935182e-4_f64) * t75907;
    let t78372 = F::cast_from(0.1276937996798935182e-4_f64) * t75910;
    let t78375 = F::cast_from(0.638468998399467591e-4_f64) * t70104;
    let t78376 = F::cast_from(0.1276937996798935182e-3_f64) * t70106;
    let t78377 = F::cast_from(0.1915406995198402773e-3_f64) * t70108;
    let t78378 = F::cast_from(0.638468998399467591e-4_f64) * t70110;
    let t78379 = F::cast_from(0.14967802127329760705e-1_f64) * t75921;
    let t78384 = F::cast_from(0.23268647941669485538e-4_f64) * t75936;
    let t78385 = F::cast_from(0.3192344991997337955e-4_f64) * t75943;
    let t78390 = t739 * t78112;
    (t78368, t78371, t78372, t78375, t78376, t78377, t78378, t78379, t78384, t78385, t78390)
}
