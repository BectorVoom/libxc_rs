//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 942/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk942<F: Float>(t73922: F, t73887: F, t73891: F, t73899: F, t73906: F, t73920: F, t73924: F, t73926: F, t73929: F, t73931: F, t76755: F, t76757: F, t76759: F, t76764: F, t76766: F, t76768: F, t76769: F) -> F {
    let t76771 = F::cast_from(0.16351352353374609375e-5_f64) * t73922;
    let t76776 = -t76755 + t73887 - F::cast_from(0.17519306092901367188e-6_f64) * t73891 + t76757 - t76759 + t76764 - F::cast_from(0.87596530464506835935e-6_f64) * t73899 - t76766 - F::cast_from(0.87596530464506835935e-6_f64) * t73906 + t76768 - t76769 - F::cast_from(0.35038612185802734376e-6_f64) * t73920 - t76771 - F::cast_from(0.81756761766873046873e-5_f64) * t73924 + F::cast_from(0.29085809927086856923e-4_f64) * t73926 - F::cast_from(0.17519306092901367187e-5_f64) * t73929 + F::cast_from(0.87596530464506835935e-6_f64) * t73931;
    t76776
}
