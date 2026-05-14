//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 857/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk857<F: Float>(t78361: F, t71346: F, t8571: F, t1981: F, t676: F, t708: F, t8512: F, t75907: F, t75910: F, t70104: F, t70106: F, t70108: F, t70110: F, t75921: F, t75936: F, t75943: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78362 = 0.12769379967989351819e-4 * t78361;
    let t78363 = t8571 * t71346;
    let t78364 = 0.85129199786595678796e-5 * t78363;
    let t78367 = t8512 * t1981 * t676 * t708;
    let t78368 = 0.42564599893297839398e-5 * t78367;
    let t78371 = 0.1276937996798935182e-4 * t75907;
    let t78372 = 0.1276937996798935182e-4 * t75910;
    let t78375 = 0.638468998399467591e-4 * t70104;
    let t78376 = 0.1276937996798935182e-3 * t70106;
    let t78377 = 0.1915406995198402773e-3 * t70108;
    let t78378 = 0.638468998399467591e-4 * t70110;
    let t78379 = 0.14967802127329760705e-1 * t75921;
    let t78384 = 0.23268647941669485538e-4 * t75936;
    let t78385 = 0.3192344991997337955e-4 * t75943;
    (t78362, t78364, t78368, t78371, t78372, t78375, t78376, t78377, t78378, t78379, t78384, t78385)
}
