//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1043/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1043<F: Float>(t73926: F, t73887: F, t73891: F, t73899: F, t73906: F, t73920: F, t73924: F, t73929: F, t73931: F, t76755: F, t76757: F, t76759: F, t76764: F, t76766: F, t76768: F, t76769: F, t76771: F) -> F {
    let t79999 = F::new(0.29085809927086856922e-4) * t73926;
    let t80002 = -t76755 + t73887 - F::new(0.17519306092901367187e-6) * t73891 + t76757 - t76759 + t76764 - F::new(0.87596530464506835932e-6) * t73899 - t76766 - F::new(0.87596530464506835932e-6) * t73906 + t76768 - t76769 - F::new(0.35038612185802734374e-6) * t73920 - t76771 - F::new(0.81756761766873046868e-5) * t73924 + t79999 - F::new(0.17519306092901367186e-5) * t73929 + F::new(0.87596530464506835932e-6) * t73931;
    t80002
}
